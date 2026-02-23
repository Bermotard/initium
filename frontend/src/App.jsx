import { useState, useEffect, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

function App() {
  const [launchers, setLaunchers] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)
  const [showModal, setShowModal] = useState(false)
  const [showSettings, setShowSettings] = useState(false)
  const [selectedLauncher, setSelectedLauncher] = useState(null)
  const [backgroundColor, setBackgroundColor] = useState(null)
  const [formData, setFormData] = useState({
    name: '',
    type: 'app',
    target: '',
    icon: ''
  })
  const fileInputRef = useRef(null)
  const backgroundFileInputRef = useRef(null)

  useEffect(() => {
    loadLaunchers()
  }, [])

  async function loadLaunchers() {
    try {
      setLoading(true)
      const result = await invoke('get_launchers')
      setLaunchers(result)
      setError(null)
    } catch (err) {
      setError(err.toString())
    } finally {
      setLoading(false)
    }
  }

  async function addLauncher(e) {
    e.preventDefault()
    try {
      if (selectedLauncher) {
        await invoke('remove_launcher_cmd', { id: selectedLauncher.id })
      }
      
      await invoke('add_launcher_cmd', {
        name: formData.name,
        launchType: formData.type,
        target: formData.target,
        icon: formData.icon || null,
      })
      setFormData({ name: '', type: 'app', target: '', icon: '' })
      setShowModal(false)
      setSelectedLauncher(null)
      await loadLaunchers()
    } catch (err) {
      setError(err.toString())
    }
  }

  async function removeLauncher(id) {
    if (window.confirm('Are you sure you want to delete this launcher?')) {
      try {
        await invoke('remove_launcher_cmd', { id })
        await loadLaunchers()
      } catch (err) {
        setError(err.toString())
      }
    }
  }

  async function executeLauncher(id) {
    try {
      const result = await invoke('execute_launcher_cmd', { id })
      console.log(result)
    } catch (err) {
      setError('Error executing launcher: ' + err.toString())
    }
  }

  async function handleExport() {
    try {
      const json = await invoke('export_config')
      const blob = new Blob([json], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const a = document.createElement('a')
      a.href = url
      a.download = 'initium-config.json'
      a.click()
      URL.revokeObjectURL(url)
    } catch (err) {
      setError(err.toString())
    }
  }

  async function handleImport(e) {
    const file = e.target.files?.[0]
    if (!file) return
    
    try {
      const json = await file.text()
      await invoke('import_config', { json })
      await loadLaunchers()
    } catch (err) {
      setError(err.toString())
    }
  }

  const handleIconUpload = (e) => {
    const file = e.target.files[0]
    if (file) {
      const reader = new FileReader()
      reader.onload = (event) => {
        setFormData({...formData, icon: event.target.result})
      }
      reader.readAsDataURL(file)
    }
  }

  const handleBackgroundImageUpload = (e) => {
    const file = e.target.files[0]
    if (file) {
      const reader = new FileReader()
      reader.onload = (event) => {
        setBackgroundColor({ type: 'image', value: event.target.result })
      }
      reader.readAsDataURL(file)
    }
  }

  const openAddModal = () => {
    setSelectedLauncher(null)
    setFormData({ name: '', type: 'app', target: '', icon: '' })
    setShowModal(true)
  }

  const openEditModal = (launcher) => {
    setSelectedLauncher(launcher)
    setFormData({
      name: launcher.name,
      type: launcher.launch_type,
      target: launcher.target,
      icon: launcher.icon || ''
    })
    setShowModal(true)
  }

  const closeModal = () => {
    setShowModal(false)
    setSelectedLauncher(null)
    setFormData({ name: '', type: 'app', target: '', icon: '' })
  }

  async function openSettingsModal() {
    try {
      const bg = await invoke('get_background')
      if (bg) {
        try {
          const parsed = JSON.parse(bg)
          setBackgroundColor(parsed)
        } catch {
          setBackgroundColor({ type: 'gradient', value: 'gradient1' })
        }
      } else {
        setBackgroundColor({ type: 'gradient', value: 'gradient1' })
      }
      setShowSettings(true)
    } catch (err) {
      setError(err.toString())
    }
  }

  async function saveBackground() {
    try {
      await invoke('set_background', { background: JSON.stringify(backgroundColor) })
      setShowSettings(false)
    } catch (err) {
      setError(err.toString())
    }
  }

  function closeSettings() {
    setShowSettings(false)
  }

  const getBackgroundStyle = () => {
    if (!backgroundColor) {
      return 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)'
    }

    if (backgroundColor.type === 'image' && backgroundColor.value) {
      return 'url(' + backgroundColor.value + ') center/cover no-repeat'
    }

    switch(backgroundColor.value) {
      case 'gradient1':
        return 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)'
      case 'gradient2':
        return 'linear-gradient(135deg, #11998e 0%, #38ef7d 100%)'
      case 'gradient3':
        return 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)'
      case 'gradient4':
        return 'linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)'
      default:
        return 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)'
    }
  }

  return (
    <div className="app" style={{background: getBackgroundStyle()}}>
      <header className="header">
        <div className="header-content">
          <h1>🚀 Initium Dashboard</h1>
          <p>Manage your application launchers</p>
        </div>
        <div className="header-actions">
          <button className="btn-add" onClick={openAddModal}>➕ Add Launcher</button>
          <button className="btn-export" onClick={handleExport}>📥 Export</button>
          <input type="file" ref={fileInputRef} onChange={handleImport} accept=".json" style={{ display: 'none' }} />
          <button className="btn-import" onClick={() => fileInputRef.current?.click()}>📤 Import</button>
          <button className="btn-settings" onClick={openSettingsModal}>⚙️</button>
        </div>
      </header>

      {error && (<div className="error-banner"><span>❌ {error}</span><button onClick={() => setError(null)}>✕</button></div>)}

      <main className="main-content">
        {loading ? (<div className="loading">Loading launchers...</div>) : launchers.length === 0 ? (<div className="empty-state"><div className="empty-icon">📭</div><h2>No Launchers Yet</h2><p>Create your first launcher to get started</p><button className="btn-add-large" onClick={openAddModal}>➕ Add Your First Launcher</button></div>) : (<div className="launchers-grid">{launchers.map((launcher) => (<div key={launcher.id} className="launcher-card"><div className="card-icon">{launcher.icon ? (<img src={launcher.icon} alt={launcher.name} />) : (<div className="icon-placeholder">{launcher.launch_type === 'web' ? '🌐' : '⚙️'}</div>)}</div><div className="card-content"><h3>{launcher.name}</h3><p className="card-type">{launcher.launch_type.toUpperCase()}</p><p className="card-target" title={launcher.target}>{launcher.target}</p></div><div className="card-actions"><button className="btn-execute" onClick={() => executeLauncher(launcher.id)}>▶</button><button className="btn-edit" onClick={() => openEditModal(launcher)}>✏️</button><button className="btn-delete" onClick={() => removeLauncher(launcher.id)}>🗑</button></div></div>))}</div>)}
      </main>

      {showModal && (<div className="modal-overlay" onClick={closeModal}><div className="modal" onClick={(e) => e.stopPropagation()}><div className="modal-header"><h2>{selectedLauncher ? 'Edit Launcher' : 'Add New Launcher'}</h2><button className="modal-close" onClick={closeModal}>✕</button></div><form onSubmit={addLauncher} className="modal-form"><div className="form-group"><label>Name</label><input type="text" placeholder="My App" value={formData.name} onChange={(e) => setFormData({...formData, name: e.target.value})} required /></div><div className="form-group"><label>Type</label><select value={formData.type} onChange={(e) => setFormData({...formData, type: e.target.value})}><option value="app">Application</option><option value="web">Website</option></select></div><div className="form-group"><label>Target (URL or Path)</label><input type="text" placeholder="/usr/bin/app or https://example.com" value={formData.target} onChange={(e) => setFormData({...formData, target: e.target.value})} required /></div><div className="form-group"><label>Icon (Optional)</label><input type="file" accept="image/*" onChange={handleIconUpload} />{formData.icon && (<div className="icon-preview"><img src={formData.icon} alt="Preview" /></div>)}</div><div className="modal-actions"><button type="submit" className="btn-submit">{selectedLauncher ? 'Update' : 'Create'} Launcher</button><button type="button" className="btn-cancel" onClick={closeModal}>Cancel</button></div></form></div></div>)}

      {showSettings && (<div className="modal-overlay" onClick={closeSettings}><div className="modal" onClick={(e) => e.stopPropagation()}><div className="modal-header"><h2>Settings</h2><button className="modal-close" onClick={closeSettings}>✕</button></div><div className="modal-form"><div className="form-group"><label>Background Type</label><select value={backgroundColor?.type || 'gradient'} onChange={(e) => { if (e.target.value === 'gradient') { setBackgroundColor({ type: 'gradient', value: 'gradient1' }) } else { setBackgroundColor({ type: 'image', value: null }) } }}><option value="gradient">Gradient</option><option value="image">Custom Image</option></select></div>{backgroundColor?.type === 'gradient' && (<div className="form-group"><label>Gradient Style</label><select value={backgroundColor.value || 'gradient1'} onChange={(e) => setBackgroundColor({ ...backgroundColor, value: e.target.value })}><option value="gradient1">Blue-Purple</option><option value="gradient2">Green</option><option value="gradient3">Pink-Red</option><option value="gradient4">Dark Blue</option></select></div>)}{backgroundColor?.type === 'image' && (<div className="form-group"><label>Upload Image</label><input type="file" accept="image/*" ref={backgroundFileInputRef} onChange={handleBackgroundImageUpload} />{backgroundColor.value && (<div className="icon-preview"><img src={backgroundColor.value} alt="Background Preview" /></div>)}</div>)}<div className="modal-actions"><button type="button" className="btn-submit" onClick={saveBackground}>Save Settings</button><button type="button" className="btn-cancel" onClick={closeSettings}>Cancel</button></div></div></div></div>)}
    </div>
  )
}

export default App