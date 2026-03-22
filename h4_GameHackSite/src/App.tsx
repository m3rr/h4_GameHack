import { BrowserRouter, Routes, Route } from 'react-router-dom'
import Home from './pages/Home'
import DetailedBreakdown from './pages/DetailedBreakdown'
import VersionHistory from './pages/VersionHistory'
import Download from './pages/Download'
import { getBasename } from './utils/router'
import { useState } from 'react'
import { AnimatePresence } from 'framer-motion'
import { SplashScreen } from './components/SplashScreen'
import { KirbySwitcher } from './components/KirbySwitcher'
import { QuoteEngine } from './components/QuoteEngine'

function App() {
  const [showSplash, setShowSplash] = useState(true)
  const basename = getBasename()

  return (
    <BrowserRouter basename={basename}>
      <AnimatePresence mode="wait">
        {showSplash && (
          <SplashScreen key="splash" onComplete={() => setShowSplash(false)} />
        )}
      </AnimatePresence>

      {!showSplash && (
        <div className="relative min-h-screen">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/details" element={<DetailedBreakdown />} />
            <Route path="/changelog" element={<VersionHistory />} />
            <Route path="/download" element={<Download />} />
          </Routes>
          <KirbySwitcher />
          <QuoteEngine />
        </div>
      )}
    </BrowserRouter>
  )
}

export default App
