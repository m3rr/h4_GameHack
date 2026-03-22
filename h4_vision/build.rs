fn main() {
    slint_build::compile("ui/main.slint").unwrap();
    
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../assets/favicon.ico");
        res.set("ProductName", "H4_Vision");
        res.set("FileDescription", "H4_Vision Memory Excavation Tool");
        res.set("LegalCopyright", "Copyright (c) 2026");
        res.set("FileVersion", "0.5.0.0");
        res.set("ProductVersion", "0.5.0.0");
        // REQUEST AS INVOKER MANIFEST (To appease Smart App Control)
        res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="asInvoker" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#);
        res.compile().unwrap();
    }
}
