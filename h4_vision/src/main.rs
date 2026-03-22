slint::include_modules!();
use h4_engine::{MemoryManager, ScriptingHost, DiscoveryEngine, ProcessScanner, ManualScanner, AOBScanner, DiscoverySignature};
use h4_engine::differential::{DifferentialScanSession, ScanOperation};
use log::info;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use slint::{Model, VecModel, SharedString, ModelRc, ComponentHandle};
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppConfig { theme_idx: i32 }
#[derive(Serialize, Deserialize, Clone, Debug)]
struct LogEntry { timestamp: String, content: String, level: String }
#[derive(Serialize, Deserialize, Clone, Debug)]
enum SentryMessage {
    Log(LogEntry),
    SetVisible(bool),
}

impl AppConfig {
    fn load() -> Self {
        fs::read_to_string("h4_config.json")
            .and_then(|c| serde_json::from_str(&c).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
            .unwrap_or(AppConfig { theme_idx: 0 })
    }
    #[allow(dead_code)]
    fn save(&self) { if let Ok(c) = serde_json::to_string_pretty(self) { let _ = fs::write("h4_config.json", c); } }
}

struct DebugLogger {
    tx: mpsc::UnboundedSender<SentryMessage>,
}

impl log::Log for DebugLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool { metadata.level() <= log::Level::Debug }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            // RECURSION SAFEGUARD: Slint/Winit internal event filtering
            if record.target().contains("slint") || record.target().contains("winit") {
                return;
            }
            
            let time_str = chrono::Local::now().format("%H:%M:%S").to_string();
            let msg = LogEntry {
                timestamp: time_str.clone(),
                content: record.args().to_string(),
                level: record.level().to_string(),
            };

            // DUAL-STREAM TELEMETRY: Echo to TTY + Pipe
            if record.level() == log::Level::Error {
                eprintln!("[{}] !! ERROR >> {}", time_str, record.args());
            } else {
                println!("[{}] >> {}", time_str, record.args());
            }
            
            let _ = self.tx.send(SentryMessage::Log(msg));
        }
    }
    fn flush(&self) {}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProfileData { theme_idx: i32, scan_results: Vec<h4_shared::ScanResult>, active_process_name: Option<String> }

fn get_profile_list() -> Vec<SharedString> {
    let mut profiles = Vec::new();
    let _ = fs::create_dir_all("profiles");
    if let Ok(entries) = fs::read_dir("profiles") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".h4ck") { profiles.push(SharedString::from(name.replace(".h4ck", ""))); }
            }
        }
    }
    profiles
}

struct FlavorTextManager { all_texts: Vec<String>, remaining: Vec<String>, first_of_last_cycle: Option<String> }
impl FlavorTextManager {
    fn new() -> Self {
        let content = fs::read_to_string("flavor_text.json").unwrap_or_else(|_| "[]".to_string());
        let mut texts: Vec<String> = serde_json::from_str(&content).unwrap_or_default();
        if texts.is_empty() { texts.push("Reticulating Splines...".to_string()); }
        Self { all_texts: texts, remaining: Vec::new(), first_of_last_cycle: None }
    }
    fn get_next(&mut self) -> String {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        if self.remaining.is_empty() {
            let mut next_pool = self.all_texts.clone();
            next_pool.shuffle(&mut rng);
            if let Some(ref last) = self.first_of_last_cycle { if next_pool[0] == *last && next_pool.len() > 1 { next_pool.swap(0, 1); } }
            self.first_of_last_cycle = Some(next_pool[0].clone());
            self.remaining = next_pool;
        }
        self.remaining.remove(0)
    }
}

fn purge_existing_instances() {
    let self_pid = std::process::id();
    // We use the engine's list_processes which is already optimized
    let processes = ProcessScanner::list_processes();
    for proc in processes {
        if (proc.name == "h4_vision.exe" || proc.name == "h4_vision") && proc.pid != self_pid {
            // WIN32 TERMINATION (NUCLEAR)
            use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
            use windows::Win32::Foundation::CloseHandle;
            unsafe {
                if let Ok(handle) = OpenProcess(PROCESS_TERMINATE, false, proc.pid) {
                    let _ = TerminateProcess(handle, 1);
                    let _ = CloseHandle(handle);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let args: Vec<String> = std::env::args().collect();
    let is_sentry = args.iter().any(|a| a == "--debug-node");
    
    if is_sentry {
        return run_sentry().await;
    }

    // PRE-FLIGHT PURGE: Clean up any stale instances before booting
    purge_existing_instances();

    // Standard Node Logger (Streams to Pipe)
    let (log_tx, mut log_rx) = mpsc::unbounded_channel::<SentryMessage>();
    let _logger = Box::leak(Box::new(DebugLogger { tx: log_tx.clone() }));
    log::set_logger(_logger).map(|()| log::set_max_level(log::LevelFilter::Debug)).unwrap();

    // PERSISTENT CRASH LOGGING & SENTRY ORPHANING
    let log_tx_panic = log_tx.clone();
    std::panic::set_hook(Box::new(move |info| {
        let msg = format!("PANIC >> SYSTEM_CORE_DUMP: {}\nLocation: {:?}", info.to_string(), info.location());
        let _ = fs::write("crash_log.txt", &msg);
        eprintln!("{}", msg);
        // Force the Sentry node to become visible before we die
        let _ = log_tx_panic.send(SentryMessage::SetVisible(true));
        let _ = log_tx_panic.send(SentryMessage::Log(LogEntry {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            content: msg.clone(),
            level: "ERROR".to_string(),
        }));
        // Small sleep to ensure the pipe has a chance to flush
        std::thread::sleep(std::time::Duration::from_millis(200));
    }));

    // Launch the Sentry process (Separate Node for Crash Persistence)
    let exe = std::env::current_exe().unwrap_or_default();
    let mut sentry_proc = std::process::Command::new(exe)
        .arg("--debug-node")
        .spawn()
        .expect("Failed to spawn Sentry Node");

    // Pipe Client Task
    tokio::spawn(async move {
        use tokio::net::windows::named_pipe::ClientOptions;
        use tokio::io::AsyncWriteExt;
        loop {
            if let Ok(mut client) = ClientOptions::new().open(r"\\.\pipe\h4_vision_debug") {
                while let Some(msg) = log_rx.recv().await {
                    let data = serde_json::to_vec(&msg).unwrap_or_default();
                    let _ = client.write_u32(data.len() as u32).await;
                    if client.write_all(&data).await.is_err() { break; }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });

    let start_time = std::time::Instant::now();
    info!("H4_Vision: Initializing futuristic interface...");
    let ui = MainWindow::new()?;

    // Debug Mode Action Bridge: Sync with Sentry Node
    let log_tx_debug = log_tx.clone();
    ui.on_debug_mode_toggled(move |enabled| {
        info!("SYSTEM >> Debug Mode Transition: {}", if enabled { "ENABLED" } else { "DISABLED" });
        let _ = log_tx_debug.send(SentryMessage::SetVisible(enabled));
    });

    // Send initial state once pipe is established
    let ui_init_node = log_tx.clone();
    let initial_visible = ui.global::<H4Themes>().get_dev_mode();
    let _ = ui_init_node.send(SentryMessage::SetVisible(initial_visible));
    info!("H4_Vision: UI created in {:?}", start_time.elapsed());
    
    // CENTER WINDOW USING WIN32 ( Centering logic )
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
        unsafe {
            let screen_w = GetSystemMetrics(SM_CXSCREEN);
            let screen_h = GetSystemMetrics(SM_CYSCREEN);
            let win_w = 1040;
            let win_h = 820;
            let pos_x = (screen_w - win_w) / 2;
            let pos_y = (screen_h - win_h) / 2;
            ui.window().set_position(slint::WindowPosition::Physical(slint::PhysicalPosition::new(pos_x, pos_y)));
        }
    }

    ui.show()?;
    info!("H4_Vision: Window displayed in {:?}", start_time.elapsed());
    
    let ui_handle = ui.as_weak();
    
    // CINEMATIC SPLASH DRIVER ( Refined Timing & Multi-Axial Logic )
    {
        let ui_anim = ui_handle.clone();
        tokio::spawn(async move {
            let sleep = |ms| tokio::time::sleep(tokio::time::Duration::from_millis(ms));
            let ui_set = ui_anim.clone();
            let ui_set_splash = ui_set.clone();
            let set_splash = move |opacity: f32, alpha: f32, j_x: f32, j_y: f32, rgb: f32| {
                let u_weak = ui_set_splash.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(u) = u_weak.upgrade() {
                        u.set_logo_opacity(opacity); u.set_glitch_alpha(alpha);
                        u.set_jitter_x(j_x); u.set_jitter_y(j_y);
                        u.set_r_offset(rgb); u.set_b_offset(-rgb);
                    }
                });
            };

            // 1.5s Solid Logo ( Presence )
            for r in 0..=50 { set_splash(r as f32 / 50.0, 0.0, 0.0, 0.0, 0.0); sleep(30).await; }
            sleep(1500).await;

            // Minor Glitch ( Beat 1 )
            for _ in 0..8 { set_splash(1.0, 0.35, rand::random::<f32>() * 12.0 - 6.0, 0.0, 4.0); sleep(20).await; }
            set_splash(1.0, 0.0, 0.0, 0.0, 0.0); sleep(800).await;

            // Severe Glitch ( Beat 2 )
            for _ in 0..12 { set_splash(1.0, 0.65, rand::random::<f32>() * 35.0 - 17.5, 0.0, 12.0); sleep(20).await; }
            set_splash(1.0, 0.0, 0.0, 0.0, 0.0); sleep(400).await;

            // Slowly Building Chaos Climax
            for i in 0..40 {
                let intensity = (i as f32 / 40.0).powi(2); // Non-linear ramp
                let alpha = if i > 30 { 0.9 } else { 0.5 * intensity };
                set_splash(1.0, alpha, rand::random::<f32>() * 80.0 * intensity, rand::random::<f32>() * 40.0 * intensity, 25.0 * intensity);
                sleep(25).await;
            }

            // FINAL IMPACT: Global Tear
            for _ in 0..15 {
                set_splash(rand::random::<f32>(), 1.0, rand::random::<f32>() * 150.0 - 75.0, rand::random::<f32>() * 60.0 - 30.0, 45.0);
                sleep(15).await;
            }

            // APP POP-IN: Jitter the main HUD for 300ms
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(u) = ui_anim.upgrade() {
                    u.set_show_splash(false);
                    u.set_app_glitch_alpha(1.0);
                }
            });

            for i in 0..15 {
                let intensity = 1.0 - (i as f32 / 15.0);
                let u_weak = ui_set.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(u) = u_weak.upgrade() {
                        u.set_app_glitch_alpha(intensity);
                        u.set_jitter_x(rand::random::<f32>() * 20.0 * intensity);
                        u.set_jitter_y(rand::random::<f32>() * 10.0 * intensity);
                    }
                });
                sleep(20).await;
            }

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(u) = ui_set.upgrade() {
                    u.set_app_glitch_alpha(0.0); u.set_jitter_x(0.0); u.set_jitter_y(0.0);
                }
            });
        });
    }

    let config = Arc::new(Mutex::new(AppConfig::load()));
    { let cfg = config.lock().unwrap(); ui.global::<H4Themes>().set_current_theme_idx(cfg.theme_idx); }
    
    // THEME CHANGE PERSISTENCE
    let config_theme = config.clone();
    ui.global::<H4Themes>().on_theme_changed(move |idx| {
        let mut cfg = config_theme.lock().unwrap();
        cfg.theme_idx = idx;
        cfg.save();
    });

    let manager_state = Arc::new(Mutex::new(Option::<MemoryManager>::None));
    let poll_addresses = Arc::new(Mutex::new(Vec::<(usize, h4_shared::ValueType)>::new()));
    let frozen_addresses = Arc::new(Mutex::new(std::collections::HashMap::<usize, u32>::new()));
    let flavor_manager = Arc::new(Mutex::new(FlavorTextManager::new()));
    let scan_start = std::time::Instant::now();
    let all_processes = Arc::new(Mutex::new(ProcessScanner::list_processes()));
    info!("H4_Vision: Initial process scan took {:?}", scan_start.elapsed());
    let all_scan_results = Arc::new(Mutex::new(Vec::<h4_shared::ScanResult>::new()));
    let scan_session = Arc::new(Mutex::new(Option::<h4_engine::differential::DifferentialScanSession>::None));

    let process_model = Rc::new(VecModel::<ProcessInfo>::default());
    let results_model = Rc::new(VecModel::<ScanEntry>::default());
    let terminal_model = Rc::new(VecModel::<TerminalLog>::default());

    ui.set_processes(ModelRc::from(process_model.clone()));
    ui.set_scan_results(ModelRc::from(results_model.clone()));
    ui.set_terminal_logs(ModelRc::from(terminal_model.clone()));
    ui.set_available_profiles(ModelRc::from(Rc::new(VecModel::from(get_profile_list()))));

    // REFRESH_RESULTS (Scope-Based Lock Isolation)
    let ui_handle = ui.as_weak();

    fn update_process_model(model: &Rc<VecModel<ProcessInfo>>, procs: &[h4_shared::ProcessEntry], filter: &str, search_term: &str) {
        let mut new_data = Vec::new();
        let term = search_term.to_lowercase();
        for p in procs {
            if (filter != "All" && p.category != filter) || (!term.is_empty() && !p.name.to_lowercase().contains(&term) && !p.pid.to_string().contains(&term)) { continue; }
            new_data.push(ProcessInfo { 
                pid: p.pid as i32, 
                name: SharedString::from(p.name.clone()), 
                status: SharedString::from(p.status.clone()), 
                is_system: p.is_system, 
                category: SharedString::from(p.category.clone()),
                is_error: false
            });
        }
        model.set_vec(new_data);
    }

    fn update_results_model(model: &Rc<VecModel<ScanEntry>>, results: &[h4_shared::ScanResult], filter: &str) {
        let mut new_data = Vec::new();
        for r in results {
            if filter != "All" && r.category != filter { continue; }
            new_data.push(ScanEntry { 
                address: SharedString::from(format!("0x{:X}", r.address)), 
                label: SharedString::from(r.label.clone().unwrap_or_default()), 
                value: SharedString::from("???"),
                value_type: SharedString::from(format!("{:?}", r.value_type))
            });
        }
        model.set_vec(new_data);
    }

    let cancel_token = Arc::new(AtomicBool::new(false));
    
    let discovery_engine = Arc::new(DiscoveryEngine::new());
    
    // STARTUP: Populate Slint Discovery Config
    let sigs_model = Rc::new(VecModel::<DiscoverySigEntry>::default());
    let sigs_data = discovery_engine.get_signatures();
    for s in sigs_data {
        sigs_model.push(DiscoverySigEntry {
            name: s.name.into(),
            aliases: s.aliases.join(", ").into(),
            patterns: s.aob_patterns.join(" | ").into(),
            ptr_offset: s.ptr_offset.map(|o| o.to_string()).unwrap_or_default().into(),
            value_type_str: format!("{:?}", s.value_type).into(),
        });
    }
    ui.set_signatures(sigs_model.clone().into());

    let procs_init = all_processes.lock().unwrap().clone();
    update_process_model(&process_model, &procs_init, "All", "");

    let ct_cancel = cancel_token.clone();
    ui.on_cancel_scan(move || {
        ct_cancel.store(true, Ordering::SeqCst);
        info!("SYSTEM >> CANCEL REQUESTED // PROXIMAL TERMINATION SIGNAL TRANSMITTED...");
    });

    let _ui_refresh = ui_handle.clone();
    let procs_refresh = all_processes.clone();
    let model_refresh = process_model.clone();
    let term_refresh = terminal_model.clone();
    ui.on_refresh_process_list(move || {
        let mut p_list = procs_refresh.lock().unwrap();
        *p_list = ProcessScanner::list_processes();
        update_process_model(&model_refresh, &p_list, "All", "");
        term_refresh.push(TerminalLog { timestamp: SharedString::from(chrono::Local::now().format("%H:%M:%S").to_string()), content: SharedString::from(format!("List refreshed. Nodes: {}", p_list.len())), level: SharedString::from("INFO") });
    });

    let procs_filter = all_processes.clone();
    let model_filter = process_model.clone();
    let term_filter = terminal_model.clone();
    let ui_filter = ui_handle.clone();
    ui.on_filter_changed(move |mode: SharedString| {
        let p_list = procs_filter.lock().unwrap();
        update_process_model(&model_filter, &p_list, mode.as_str(), "");
        if let Some(u) = ui_filter.upgrade() { u.invoke_refresh_results(); }
        term_filter.push(TerminalLog { timestamp: SharedString::from(chrono::Local::now().format("%H:%M:%S").to_string()), content: SharedString::from(format!("Global Filter: {}", mode)), level: SharedString::from("INFO") });
    });

    let procs_search = all_processes.clone();
    let model_search = process_model.clone();
    ui.on_search_term_changed(move |term: SharedString, mode: SharedString| {
        let p_list = procs_search.lock().unwrap();
        update_process_model(&model_search, &p_list, mode.as_str(), term.as_str());
    });

    let model_refresh_results = results_model.clone();
    let res_store_refresh = all_scan_results.clone();
    let ui_refresh_results = ui_handle.clone();
    ui.on_refresh_results(move || {
        if let Some(u) = ui_refresh_results.upgrade() {
            let filter = u.get_filter_mode();
            update_results_model(&model_refresh_results, &res_store_refresh.lock().unwrap(), filter.as_str());
        }
    });

    let ui_ptr = ui_handle.clone();
    let res_ptr = all_scan_results.clone();
    let poll_ptr = poll_addresses.clone();
    let mgr_ptr = manager_state.clone();
    let ct_ptr = cancel_token.clone();
    ui.on_find_pointers(move |addr_str: SharedString| {
        info!("SYSTEM >> Pointer Excavation Initiated for: {}", addr_str);
        if let Some(u) = ui_ptr.upgrade() {
            u.set_scanning_active(true); u.set_scan_status(SharedString::from("EXCAVATING POINTERS..."));
            let m = mgr_ptr.clone(); let r = res_ptr.clone(); let p = poll_ptr.clone();
            let ui_inner = ui_ptr.clone(); let ct = ct_ptr.clone();
            let a_val = usize::from_str_radix(addr_str.as_str().trim_start_matches("0x"), 16).unwrap_or(0);
            
            tokio::spawn(async move {
                let mgr_clone = { let st = m.lock().unwrap(); st.as_ref().map(|mgr| mgr.clone()) };
                let found = if let Some(mgr) = mgr_clone {
                    ManualScanner::find_pointers(&mgr, a_val, ct)
                } else { Vec::new() };

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(u) = ui_inner.upgrade() {
                        {
                            let mut res_m = r.lock().unwrap();
                            let mut poll_m = p.lock().unwrap();
                            res_m.clear(); poll_m.clear();
                            for addr in found.iter().take(200) {
                                res_m.push(h4_shared::ScanResult { 
                                    address: *addr, 
                                    label: Some(format!("Pointer -> 0x{:X}", a_val)), 
                                    value_type: h4_shared::ValueType::Int64, 
                                    category: "Manual".to_string() 
                                });
                                poll_m.push((*addr, h4_shared::ValueType::Int64));
                            }
                        }
                        u.invoke_refresh_results(); u.set_scanning_active(false);
                        u.set_scan_status(SharedString::from(format!("POINTER SCAN COMPLETE: {} PATHS.", found.len())));
                    }
                });
            });
        }
    });

    let model_search_ws = results_model.clone();
    let res_store_search = all_scan_results.clone();
    let ui_search_ws = ui_handle.clone();
    ui.on_workspace_search(move |query: SharedString| {
        if let Some(u) = ui_search_ws.upgrade() {
            let results = res_store_search.lock().unwrap();
            let query_lower = query.to_lowercase();
            
            let filtered: Vec<h4_shared::ScanResult> = results.iter()
                .filter(|r| {
                    r.label.as_ref().map(|l| l.to_lowercase().contains(&query_lower)).unwrap_or(false) ||
                    format!("0x{:X}", r.address).to_lowercase().contains(&query_lower)
                })
                .cloned()
                .collect();

            if filtered.is_empty() && !query.is_empty() {
                u.set_search_error_open(true);
                let u_weak = ui_search_ws.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                    let _ = slint::invoke_from_event_loop(move || { if let Some(u) = u_weak.upgrade() { u.set_search_error_open(false); } });
                });
            } else {
                update_results_model(&model_search_ws, &filtered, u.get_filter_mode().as_str());
            }
        }
    });

    let ui_select = ui_handle.clone();
    let m_state_select = manager_state.clone();
    let term_select = terminal_model.clone();
    let res_select = all_scan_results.clone();
    let poll_select = poll_addresses.clone();
    let results_model_select = results_model.clone();
    let process_model_select = process_model.clone();
    ui.on_process_selected(move |pid| {
        if pid == -1 {
            *m_state_select.lock().unwrap() = None;
            res_select.lock().unwrap().clear();
            poll_select.lock().unwrap().clear();
            while results_model_select.row_count() > 0 { results_model_select.remove(0); }
            term_select.push(TerminalLog { 
                timestamp: SharedString::from(chrono::Local::now().format("%H:%M:%S").to_string()), 
                content: SharedString::from("System Detached. Node cleared."),
                level: SharedString::from("WARN")
            });
            return;
        }
        if let Some(u) = ui_select.upgrade() {
            // Reset previous errors in model
            for i in 0..process_model_select.row_count() {
                if let Some(mut p) = process_model_select.row_data(i) {
                    if p.is_error { p.is_error = false; process_model_select.set_row_data(i, p); }
                }
            }

            match MemoryManager::attach(pid as u32) {
                Ok(mgr) => {
                    let name = mgr.get_process_name().unwrap_or_default();
                    u.set_active_pid(pid); u.set_active_process_name(SharedString::from(name.clone()));
                    *m_state_select.lock().unwrap() = Some(mgr);
                    term_select.push(TerminalLog { timestamp: SharedString::from(chrono::Local::now().format("%H:%M:%S").to_string()), content: SharedString::from(format!("Attached: {} (PID: {})", name, pid)), level: SharedString::from("INFO") });
                }
                Err(e) => {
                    // Turn PID red in model
                    for i in 0..process_model_select.row_count() {
                        if let Some(mut p) = process_model_select.row_data(i) {
                            if p.pid == pid { p.is_error = true; process_model_select.set_row_data(i, p); break; }
                        }
                    }
                    u.set_active_pid(-1);
                    term_select.push(TerminalLog { timestamp: SharedString::from(chrono::Local::now().format("%H:%M:%S").to_string()), content: SharedString::from(format!("ACCESS DENIED: {}", e)), level: SharedString::from("ERROR") });
                }
            }
        }
    });

    let (tx, mut rx) = mpsc::unbounded_channel::<(usize, String)>();
    let m_state_manual = manager_state.clone();
    let res_manual = all_scan_results.clone();
    let poll_manual = poll_addresses.clone();
    let ui_manual = ui_handle.clone();
    let flavors_manual = flavor_manager.clone();
    let procs_manual = all_processes.clone();
    let cancel_manual = cancel_token.clone();
    let session_manual = scan_session.clone();
    
    ui.on_start_manual_search(move |val: SharedString, size: SharedString, _crit: SharedString, _type_str: SharedString| {
        info!("SYSTEM >> Engagement Initiated: MANUAL_EXCAVATION (Value: {}, Size: {})", val, size);
        if let Some(u) = ui_manual.upgrade() {
            u.set_scanning_active(true); u.set_scan_progress(0.0);
            let m = m_state_manual.clone(); let r = res_manual.clone(); let p = poll_manual.clone();
            let ui_inner = ui_manual.clone(); let f = flavors_manual.clone();
            let p_list_inner = procs_manual.clone();
            let v_str = val.to_string(); let s_str = size.to_string();
            let ct = cancel_manual.clone();
            let sess = session_manual.clone();
            ct.store(false, Ordering::SeqCst);

            let mgr_clone = { let st = m.lock().unwrap(); st.as_ref().map(|mgr| mgr.clone()) };

            tokio::spawn(async move {
                let status = f.lock().unwrap().get_next();
                let ui_status = ui_inner.clone();
                let _ = slint::invoke_from_event_loop(move || { if let Some(u) = ui_status.upgrade() { u.set_scan_status(SharedString::from(status)); u.set_scan_progress(0.3); } });
                
                let (found, vt) = if let Some(mgr) = mgr_clone.clone() {
                    let mut a = Vec::new(); let mut vtt = h4_shared::ValueType::Int32;
                    match s_str.as_str() {
                        "1 Byte" => if let Ok(vv) = v_str.parse::<u8>() { a = ManualScanner::scan_for_value(&mgr, vv, ct); vtt = h4_shared::ValueType::Byte; }
                        "2 Bytes" => if let Ok(vv) = v_str.parse::<i16>() { a = ManualScanner::scan_for_value(&mgr, vv, ct); vtt = h4_shared::ValueType::Int16; }
                        "4 Bytes" => if let Ok(vv) = v_str.parse::<i32>() { a = ManualScanner::scan_for_value(&mgr, vv, ct); vtt = h4_shared::ValueType::Int32; }
                        "8 Bytes" => if let Ok(vv) = v_str.parse::<i64>() { a = ManualScanner::scan_for_value(&mgr, vv, ct); vtt = h4_shared::ValueType::Int64; }
                        "Float" => if let Ok(vv) = v_str.parse::<f32>() { a = ManualScanner::scan_for_value(&mgr, vv, ct); vtt = h4_shared::ValueType::Float32; }
                        "Double" => if let Ok(vv) = v_str.parse::<f64>() { a = ManualScanner::scan_for_value(&mgr, vv, ct); vtt = h4_shared::ValueType::Float64; }
                        "Custom AOB" => {
                            if let Ok(scanner) = AOBScanner::new(&v_str) { a = scanner.scan_process(&mgr, ct.clone()); }
                            vtt = h4_shared::ValueType::AOB;
                        }
                        _ => if let Ok(vv) = v_str.parse::<i32>() { a = ManualScanner::scan_for_value(&mgr, vv, ct); }
                    }
                    if !a.is_empty() {
                        let mut sess_lock = sess.lock().unwrap();
                        *sess_lock = Some(DifferentialScanSession::new(a.clone(), &mgr, vtt.clone()));
                    }
                    (a, vtt)
                } else { (Vec::new(), h4_shared::ValueType::Int32) };
                
                let ui_final = ui_inner.clone();
                let p_list = p_list_inner.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(u) = ui_final.upgrade() {
                        {
                            let mut res_m = r.lock().unwrap();
                            let mut poll_m = p.lock().unwrap();
                            res_m.clear(); poll_m.clear();
                            let active_pid = u.get_active_pid();
                            let active_cat = {
                                let procs = p_list.lock().unwrap();
                                procs.iter().find(|p| p.pid as i32 == active_pid).map(|p| p.category.clone()).unwrap_or("Third Party".to_string())
                            };
                            for addr in found.iter().take(200) {
                                res_m.push(h4_shared::ScanResult { address: *addr, label: Some("Initial Findings".to_string()), value_type: vt.clone(), category: active_cat.clone() });
                                poll_m.push((*addr, vt.clone()));
                            }
                        }
                        u.invoke_refresh_results(); u.set_scanning_active(false); u.set_scan_session_active(true);
                        u.set_scan_status(SharedString::from(format!("EXCAVATION COMPLETE: {} CANDIDATES.", found.len())));
                    }
                });
            });
        }
    });

    let ui_next = ui_handle.clone();
    let sess_next = scan_session.clone();
    let mgr_next = manager_state.clone();
    let res_next = all_scan_results.clone();
    let poll_next = poll_addresses.clone();
    let cancel_next = cancel_token.clone();
    ui.on_next_manual_scan(move |crit, val, _size| {
        info!("SYSTEM >> Iterative Re-Scan: Filtering with criteria '{}'", crit);
        if let Some(u) = ui_next.upgrade() {
            u.set_scanning_active(true);
            let sess = sess_next.clone(); let m = mgr_next.clone(); let r = res_next.clone(); 
            let p = poll_next.clone(); let ui_inner = ui_next.clone();
            let c_str = crit.to_string(); let v_str = val.to_string();
            let ct = cancel_next.clone(); ct.store(false, Ordering::SeqCst);

            tokio::spawn(async move {
                let mgr = { let st = m.lock().unwrap(); st.as_ref().map(|mgr| mgr.clone()) };
                let op = match c_str.as_str() {
                    "Increased" => ScanOperation::Increased,
                    "Decreased" => ScanOperation::Decreased,
                    "Changed" => ScanOperation::Changed,
                    "Unchanged" => ScanOperation::Unchanged,
                    _ => ScanOperation::ExactValue,
                };
                
                let (found, vt) = if let (Some(mgr), Some(sess_lock)) = (mgr, sess.lock().unwrap().as_mut()) {
                    let target_val = match sess_lock.value_type {
                        h4_shared::ValueType::Int32 => v_str.parse::<i32>().ok().map(|v| v.to_le_bytes().to_vec()),
                        h4_shared::ValueType::Float32 => v_str.parse::<f32>().ok().map(|v| v.to_le_bytes().to_vec()),
                        h4_shared::ValueType::Float64 => v_str.parse::<f64>().ok().map(|v| v.to_le_bytes().to_vec()),
                        _ => None,
                    };
                    sess_lock.filter(&mgr, op, target_val, ct);
                    (sess_lock.candidates.clone(), sess_lock.value_type.clone())
                } else { (Vec::new(), h4_shared::ValueType::Int32) };

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(u) = ui_inner.upgrade() {
                        {
                            let mut res_m = r.lock().unwrap();
                            let mut poll_m = p.lock().unwrap();
                            res_m.clear(); poll_m.clear();
                            for addr in found.iter().take(200) {
                                res_m.push(h4_shared::ScanResult { address: *addr, label: Some("Candidate (Refined)".to_string()), value_type: vt.clone(), category: "Manual".to_string() });
                                poll_m.push((*addr, vt.clone()));
                            }
                        }
                        u.invoke_refresh_results(); u.set_scanning_active(false);
                        u.set_scan_status(SharedString::from(format!("FILTER COMPLETE: {} REMAINING.", found.len())));
                    }
                });
            });
        }
    });

    let ui_reset = ui_handle.clone();
    let sess_reset = scan_session.clone();
    ui.on_reset_scan_session(move || {
        let mut sess = sess_reset.lock().unwrap();
        *sess = None;
        if let Some(u) = ui_reset.upgrade() {
            u.set_scan_session_active(false);
            u.set_scan_status(SharedString::from("SCAN SESSION RESET. READY FOR NEW PASS."));
        }
    });

    ui.on_debug_mode_toggled(move |enabled| {
        info!("SYSTEM >> Debug Mode Transition: {}", if enabled { "ENABLED" } else { "DISABLED" });
    });

    let m_state_smart = manager_state.clone();
    let discovery = discovery_engine.clone();
    let res_smart = all_scan_results.clone();
    let poll_smart = poll_addresses.clone();
    let ui_smart = ui_handle.clone();
    let _flavors_smart = flavor_manager.clone();
    
    let discovery_targeted = discovery.clone();
    let ct_targeted_base = cancel_token.clone();
    ui.on_targeted_smart_search(move |target: SharedString, type_str: SharedString| {
        info!("SYSTEM >> Engagement Initiated: Targeted Discovery (Target: {}, Type: {})", target, type_str);
        if let Some(u) = ui_smart.upgrade() {
            u.set_scanning_active(true);
            let m = m_state_smart.clone(); let d = discovery_targeted.clone(); let r = res_smart.clone();
            let p = poll_smart.clone(); let ui_inner = ui_smart.clone();
            let t_str = target.to_string(); let ct_targeted = ct_targeted_base.clone();
            let t_type = type_str.to_string();
            
            // PROXIMAL OPTIMIZATION: Clone the manager to avoid holding the global lock
            info!("SYSTEM >> Sequence Transition: Replicating Memory Handler for isolated context...");
            let mgr_clone = { let st = m.lock().unwrap(); st.as_ref().map(|mgr| mgr.clone()) };
            
            if let Some(mgr) = mgr_clone {
                info!("SYSTEM >> Background Handover: Spawning Discovery Thread...");
                tokio::spawn(async move {
                    let ui_status = ui_inner.clone();
                    let t_str_clone = t_str.clone();
                    let t_type_clone = t_type.clone();
                    let _ = slint::invoke_from_event_loop(move || { if let Some(u) = ui_status.upgrade() { u.set_scan_status(SharedString::from(format!("Scanning for {} ({})...", t_str_clone, t_type_clone))); u.set_scan_progress(0.2); } });
                    
                    let ct = ct_targeted.clone();
                    ct.store(false, Ordering::SeqCst);
                    let discovered = d.targeted_scan(&mgr, &t_str, &t_type, ct);
                
                let ui_final = ui_inner.clone();
                let msg = if discovered.is_empty() { format!("Didn't find any {} :(", t_str) } else { format!("FOUND {} {}! :D", discovered.len(), t_str) };

                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(u) = ui_final.upgrade() {
                        { // ISOLATION BLOCK: Ensure mutex locks are released before UI callbacks trigger
                            let mut res_m = r.lock().unwrap();
                            let mut poll_m = p.lock().unwrap();
                            *res_m = discovered.clone(); 
                            poll_m.clear();
                            for item in &*res_m { 
                                poll_m.push((item.address, item.value_type.clone())); 
                            }
                        } // Mutex release occurs here
                        
                        u.set_scan_status(SharedString::from(msg));
                        u.set_scan_progress(1.0);
                        u.invoke_refresh_results(); // Safe: No longer holding locks during recursive callback
                        u.set_scanning_active(false);
                    }
                });
            });
        }
    }
});

    let ct_confirm = cancel_token.clone();
    let ui_confirm = ui_handle.clone();
    let m_state_confirm = manager_state.clone();
    let disc_engine_confirm = discovery.clone();
    ui.on_confirm_smart_scan(move || {
        info!("SYSTEM >> Engagement Initiated: SMART_SCAN_SEQUENCE");
        if let Some(u) = ui_confirm.upgrade() {
            u.set_scanning_active(true); u.set_scan_status(slint::SharedString::from("EXCAVATING..."));
            u.set_scan_progress(0.1);
            let m = m_state_confirm.clone();
            let disc = disc_engine_confirm.clone();
            let ui_inner = ui_confirm.clone();
            let ct = ct_confirm.clone();
            ct.store(false, Ordering::SeqCst);

            // PROXIMAL OPTIMIZATION: Clone manager to release lock
            info!("SYSTEM >> Sequence Transition: Replicating Memory Handler...");
            let mgr_clone = { let st = m.lock().unwrap(); st.as_ref().map(|mgr| mgr.clone()) };
            
            if let Some(mgr) = mgr_clone {
                info!("SYSTEM >> Background Handover: Spawning Smart Scan Task...");
                tokio::spawn(async move {
                    let discovered = disc.smart_scan(&mgr, ct);
                    
                    let _ = slint::invoke_from_event_loop(move || {
                    if let Some(u) = ui_inner.upgrade() {
                        let mut final_results: Vec<ScanEntry> = Vec::new();
                        for res in discovered {
                            final_results.push(ScanEntry {
                                address: format!("{:X}", res.address).into(),
                                label: res.label.unwrap_or_default().into(),
                                value: "???".into(),
                                value_type: format!("{:?}", res.value_type).into(),
                            });
                        }
                        u.set_scan_results(Rc::new(VecModel::from(final_results)).into());
                        u.set_scanning_active(false);
                        u.set_scan_progress(1.0);
                        u.set_scan_status(slint::SharedString::from("EXCAVATION COMPLETE."));
                    }
                });
            });
        }
    }
});

    let ct_cancel_final = cancel_token.clone();    let disc_engine_add = discovery_engine.clone();
    let sigs_model_add = sigs_model.clone();
    ui.on_add_custom_signature(move |name, aliases, patterns, ptr_off| {
        let p_off = ptr_off.parse::<i32>().ok();
        let sig = DiscoverySignature {
            name: name.to_string(),
            aliases: aliases.split(',').map(|s| s.trim().to_string()).collect(),
            aob_patterns: patterns.split('|').map(|s| s.trim().to_string()).collect(),
            offset: 0,
            ptr_offset: p_off,
            value_type: h4_shared::ValueType::Int32,
            category: "Custom".to_string(),
        };
        disc_engine_add.add_signature(sig.clone());
        sigs_model_add.push(DiscoverySigEntry {
            name: name,
            aliases: aliases,
            patterns: patterns,
            ptr_offset: ptr_off,
            value_type_str: "Int32".into(),
        });
    });
    ui.on_cancel_smart_scan(move || {
        ct_cancel_final.store(true, Ordering::SeqCst);
        info!("SYSTEM >> Terminal Signal: Termination of scan initiated by user.");
    });


    let ui_manual_req = ui_handle.clone();
    let res_manual_req = all_scan_results.clone();
    let poll_manual_req = poll_addresses.clone();
    ui.on_manual_scan_requested(move |addr, type_str, _val| {
        let addr_val = if addr.to_lowercase().starts_with("0x") {
            usize::from_str_radix(&addr[2..].trim(), 16).unwrap_or(0)
        } else {
            usize::from_str_radix(&addr.trim(), 16).unwrap_or(0)
        };
        
        let vt = match type_str.to_lowercase().as_str() {
            "byte" | "1 byte" => h4_shared::ValueType::Byte,
            "int16" | "2 bytes" => h4_shared::ValueType::Int16,
            "int32" | "4 bytes" => h4_shared::ValueType::Int32,
            "int64" | "8 bytes" => h4_shared::ValueType::Int64,
            "float" | "float32" => h4_shared::ValueType::Float32,
            "double" | "float64" => h4_shared::ValueType::Float64,
            _ => h4_shared::ValueType::Int32,
        };
        
        info!("SYSTEM >> MANUAL TRACK: Intercepting address 0x{:X} as {:?}", addr_val, vt);
        
        let mut rm = res_manual_req.lock().unwrap();
        let mut pm = poll_manual_req.lock().unwrap();
        
        rm.push(h4_shared::ScanResult {
            address: addr_val,
            label: Some("Manual".to_string()),
            value_type: vt.clone(),
            category: "Third Party".to_string(),
        });
        pm.push((addr_val, vt));
        
        if let Some(u) = ui_manual_req.upgrade() {
            u.invoke_refresh_results();
        }
    });

    ui.on_close_discovery_config(move || {
        info!("SYSTEM >> Signature Database access terminated.");
    });

    let ui_save = ui_handle.clone();
    let res_save = all_scan_results.clone();
    ui.on_save_profile(move |name: SharedString| {
        if name.is_empty() { return; }
        if let Some(u) = ui_save.upgrade() {
            let data = ProfileData { theme_idx: u.global::<H4Themes>().get_current_theme_idx(), scan_results: res_save.lock().unwrap().clone(), active_process_name: Some(u.get_active_process_name().to_string()) };
            if let Ok(c) = serde_json::to_string_pretty(&data) { let _ = fs::write(format!("profiles/{}.h4ck", name), c); }
        }
    });

    let ui_load = ui_handle.clone();
    let res_load = all_scan_results.clone();
    let poll_load = poll_addresses.clone();
    let _model_load = results_model.clone();
    ui.on_load_profile(move |name: SharedString| {
        if let Some(u) = ui_load.upgrade() {
            if let Ok(c) = fs::read_to_string(format!("profiles/{}.h4ck", name)) {
                if let Ok(d) = serde_json::from_str::<ProfileData>(&c) {
                    u.global::<H4Themes>().set_current_theme_idx(d.theme_idx);
                    // Update global app config as well
                    if let Ok(mut cfg) = config.lock() {
                        cfg.theme_idx = d.theme_idx;
                        cfg.save();
                    }
                    let (mut rm, mut pm) = (res_load.lock().unwrap(), poll_load.lock().unwrap());
                    rm.clear(); pm.clear(); 
                    for r in d.scan_results {
                        pm.push((r.address, r.value_type.clone()));
                        rm.push(r.clone());
                    }
                    u.invoke_refresh_results();
                }
            }
        }
    });

    let m_state_term = manager_state.clone();
    let term_model_term = terminal_model.clone();
    let frozen_term = frozen_addresses.clone();
    ui.on_terminal_command(move |cmd: SharedString| {
        if let Some(mgr) = m_state_term.lock().unwrap().as_ref() {
            let res = ScriptingHost::dispatch(&cmd, mgr);
            
            // h4: Check for result that implies state change (like freeze)
            if cmd.starts_with("h4 --freeze") {
                let parts: Vec<&str> = cmd.split_whitespace().collect();
                if parts.len() >= 3 {
                    if let Ok(addr) = usize::from_str_radix(parts[2].trim_start_matches("0x"), 16) {
                        if let Ok(val) = mgr.read::<u32>(addr) {
                            frozen_term.lock().unwrap().insert(addr, val);
                            info!("STATE >> Persistent strobe applied to 0x{:X} (Value: {})", addr, val);
                        }
                    }
                }
            }
            
            term_model_term.push(TerminalLog { timestamp: SharedString::from(chrono::Local::now().format("%H:%M:%S").to_string()), content: SharedString::from(res), level: SharedString::from("INFO") });
        }
    });

    ui.on_close_requested(move || { let _ = slint::quit_event_loop(); });
    let ui_min = ui_handle.clone();
    ui.on_minimize_requested(move || { if let Some(u) = ui_min.upgrade() { u.window().set_minimized(true); } });
    let ui_max = ui_handle.clone();
    ui.on_maximize_requested(move || { if let Some(u) = ui_max.upgrade() { let m = u.window().is_maximized(); u.window().set_maximized(!m); } });
    let ui_drag = ui_handle.clone();
    ui.on_window_dragged(move |ox, oy| { if let Some(u) = ui_drag.upgrade() { let p = u.window().position(); u.window().set_position(slint::WindowPosition::Physical(slint::PhysicalPosition::new(p.x + ox as i32, p.y + oy as i32))); } });

    let m_state_p = manager_state.clone();
    let poll_p = poll_addresses.clone();
    let frozen_p = frozen_addresses.clone();
    let tx_p = tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));
        loop {
            interval.tick().await;
            if let Ok(mgr_lock) = m_state_p.try_lock() { // Use try_lock
                if let Some(mgr) = mgr_lock.as_ref() {
                    // h4: Enforce frozen addresses (The Strobe)
                    if let Ok(frozen) = frozen_p.try_lock() {
                        for (addr, val) in frozen.iter() {
                            let _ = mgr.write::<u32>(*addr, *val);
                        }
                    }
                    
                    if let Ok(addrs_lock) = poll_p.try_lock() { // Use try_lock
                        for i in 0..std::cmp::min(addrs_lock.len(), 50) {
                            let (addr, vt) = addrs_lock[i].clone(); // Clone to avoid holding lock
                            let val_str = match vt {
                                h4_shared::ValueType::Byte => mgr.read::<u8>(addr).map(|v| v.to_string()),
                                h4_shared::ValueType::Int16 => mgr.read::<i16>(addr).map(|v| v.to_string()),
                                h4_shared::ValueType::Int32 => mgr.read::<i32>(addr).map(|v| v.to_string()),
                                h4_shared::ValueType::Int64 => mgr.read::<i64>(addr).map(|v| v.to_string()),
                                h4_shared::ValueType::Float32 => mgr.read::<f32>(addr).map(|v| format!("{:.2}", v)),
                                h4_shared::ValueType::Float64 => mgr.read::<f64>(addr).map(|v| format!("{:.2}", v)),
                                _ => Ok("???".to_string()),
                            };
                            if let Ok(v) = val_str { let _ = tx_p.send((i, v)); }
                        }
                    }
                }
            }
        }
    });

    let res_m = results_model.clone();
    let _timer = slint::Timer::default();
    _timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
        while let Ok((i, v)) = rx.try_recv() { if let Some(mut e) = res_m.row_data(i) { e.value = SharedString::from(v); res_m.set_row_data(i, e); } }
    });

    let run_res = ui.run();
    
    // SHUTDOWN SEQUENCE: Orbital Purge
    let _ = sentry_proc.kill();
    info!("SYSTEM >> Shutdown Sequence: Sentry Node Purged. Cleaning workspace...");
    purge_existing_instances();
    
    run_res
}

async fn run_sentry() -> Result<(), slint::PlatformError> {
    use tokio::net::windows::named_pipe::ServerOptions;
    use tokio::io::AsyncReadExt;
    
    eprintln!("H4_SENTRY: System Node booting...");
    let ui = DebugWindow::new()?;
    eprintln!("H4_SENTRY: UI initialized. Listening on pipe h4_vision_debug...");
    let logs_model = Rc::new(VecModel::<TerminalLog>::default());
    ui.set_logs(ModelRc::from(logs_model.clone()));
    
    let ui_handle = ui.as_weak();
    let dbg_model_ctrl = logs_model.clone();
    
    ui.on_clear_logs(move || { dbg_model_ctrl.set_vec(Vec::new()); });
    
    let dbg_model_copy = logs_model.clone();
    ui.on_copy_all(move || {
        let logs: Vec<String> = (0..dbg_model_copy.row_count()).filter_map(|i| dbg_model_copy.row_data(i).map(|d| format!("[{}] {}", d.timestamp, d.content))).collect();
        let _ = fs::write("last_copy.txt", logs.join("\n"));
    });

    let dbg_model_errors = logs_model.clone();
    ui.on_copy_errors(move || {
        let logs: Vec<String> = (0..dbg_model_errors.row_count()).filter_map(|i| dbg_model_errors.row_data(i)).filter(|d| d.level == "ERROR").map(|d| format!("[{}] {}", d.timestamp, d.content)).collect();
        let _ = fs::write("last_errors.txt", logs.join("\n"));
    });

    let dbg_model_md = logs_model.clone();
    ui.on_export_md(move || {
        let mut md = String::from("# H4_Sentry Debug Report\n\n| Timestamp | Level | Message |\n| :--- | :--- | :--- |\n");
        for i in 0..dbg_model_md.row_count() { if let Some(d) = dbg_model_md.row_data(i) { md.push_str(&format!("| {} | **{}** | `{}` |\n", d.timestamp, d.level, d.content)); } }
        let _ = fs::write("debug_report.md", md);
    });

    tokio::spawn(async move {
        let server_options = ServerOptions::new();
        loop {
            if let Ok(mut server) = server_options.create(r"\\.\pipe\h4_vision_debug") {
                let _ = server.connect().await;
                loop {
                    let mut len_buf = [0u8; 4];
                    if server.read_exact(&mut len_buf).await.is_err() { break; }
                    let len = u32::from_le_bytes(len_buf) as usize;
                    let mut data = vec![0u8; len];
                    if server.read_exact(&mut data).await.is_err() { break; }
                    
                    if let Ok(msg) = serde_json::from_slice::<SentryMessage>(&data) {
                        let u_weak = ui_handle.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(u) = u_weak.upgrade() {
                                match msg {
                                    SentryMessage::Log(log) => {
                                        let model = u.get_logs();
                                        if let Some(m) = model.as_any().downcast_ref::<VecModel<TerminalLog>>() {
                                            m.push(TerminalLog {
                                                timestamp: SharedString::from(log.timestamp),
                                                content: SharedString::from(log.content),
                                                level: SharedString::from(log.level),
                                            });
                                        }
                                    },
                                    SentryMessage::SetVisible(visible) => {
                                        if visible { let _ = u.show(); } else { let _ = u.window().hide(); }
                                    }
                                }
                            }
                        });
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    slint::run_event_loop()
}
