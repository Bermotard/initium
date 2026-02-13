import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

function App() {
  const [launchers, setLaunchers] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)
  const [showModal, setShowModal] = useState(false)
  const [selectedLauncher, setSelectedLauncher] = useState(null)
  const [formData, setFormData] = useState({
    id: '',
    name: '',
    type: 'app',
    target: '',
    icon: ''
  })

  // Load launchers on mount
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
    // Si on édite, supprimer l'ancien d'abord
    if (selectedLauncher) {
      await invoke('remove_launcher_cmd', { id: selectedLauncher.id })
    }
    
    // Puis ajouter le nouveau
    await invoke('add_launcher_cmd', {
      id: formData.id,
      name: formData.name,
      launchType: formData.type,
      target: formData.target,
      icon: formData.icon || null,
    })
      setFormData({ id: '', name: '', type: 'app', target: '', icon: '' })
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

  const openAddModal = () => {
    setSelectedLauncher(null)
    setFormData({ id: '', name: '', type: 'app', target: '', icon: '' })
    setShowModal(true)
  }

  const openEditModal = (launcher) => {
    setSelectedLauncher(launcher)
    setFormData({
      id: launcher.id,
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
    setFormData({ id: '', name: '', type: 'app', target: '', icon: '' })
  }

  return (
    <div className="app">
      {/* Header */}
      <header className="header">
        <div className="header-content">
          <h1>🚀 Initium Dashboard</h1>
          <p>Manage your application launchers</p>
        </div>
        <div className="header-actions">
          <button 
            className="btn-add"
            onClick={openAddModal}
          >
            ➕ Add Launcher
          </button>
          <button className="btn-settings">⚙️</button>
        </div>
      </header>

      {/* Error Message */}
      {error && (
        <div className="error-banner">
          <span>❌ {error}</span>
          <button onClick={() => setError(null)}>✕</button>
        </div>
      )}

      {/* Main Content */}
      <main className="main-content">
        {loading ? (
          <div className="loading">Loading launchers...</div>
        ) : launchers.length === 0 ? (
          <div className="empty-state">
            <div className="empty-icon">📭</div>
            <h2>No Launchers Yet</h2>
            <p>Create your first launcher to get started</p>
            <button 
              className="btn-add-large"
              onClick={openAddModal}
            >
              ➕ Add Your First Launcher
            </button>
          </div>
        ) : (
          <div className="launchers-grid">
            {launchers.map((launcher) => (
              <div key={launcher.id} className="launcher-card">
                <div className="card-icon">
                  {launcher.icon ? (
                    <img src={launcher.icon} alt={launcher.name} />
                  ) : (
                    <div className="icon-placeholder">
                      {launcher.launch_type === 'web' ? '🌐' : '⚙️'}
                    </div>
                  )}
                </div>
                <div className="card-content">
                  <h3>{launcher.name}</h3>
                  <p className="card-type">{launcher.launch_type.toUpperCase()}</p>
                  <p className="card-target" title={launcher.target}>{launcher.target}</p>
                </div>
                <div className="card-actions">
                  <button 
                    className="btn-execute"
                    onClick={() => executeLauncher(launcher.id)}
                    title="Execute launcher"
                  >
                    ▶
                  </button>
                  <button 
                    className="btn-edit"
                    onClick={() => openEditModal(launcher)}
                    title="Edit launcher"
                  >
                    ✏️
                  </button>
                  <button 
                    className="btn-delete"
                    onClick={() => removeLauncher(launcher.id)}
                    title="Delete launcher"
                  >
                    🗑
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>

      {/* Modal Add/Edit Launcher */}
      {showModal && (
        <div className="modal-overlay" onClick={closeModal}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>{selectedLauncher ? 'Edit Launcher' : 'Add New Launcher'}</h2>
              <button 
                className="modal-close"
                onClick={closeModal}
              >
                ✕
              </button>
            </div>

            <form onSubmit={addLauncher} className="modal-form">
              <div className="form-group">
                <label>ID</label>
                <input
                  type="text"
                  placeholder="launcher-id"
                  value={formData.id}
                  onChange={(e) => setFormData({...formData, id: e.target.value})}
                  disabled={selectedLauncher !== null}
                  required
                />
              </div>

              <div className="form-group">
                <label>Name</label>
                <input
                  type="text"
                  placeholder="My App"
                  value={formData.name}
                  onChange={(e) => setFormData({...formData, name: e.target.value})}
                  required
                />
              </div>

              <div className="form-group">
                <label>Type</label>
                <select
                  value={formData.type}
                  onChange={(e) => setFormData({...formData, type: e.target.value})}
                >
                  <option value="app">Application</option>
                  <option value="web">Website</option>
                </select>
              </div>

              <div className="form-group">
                <label>Target (URL or Path)</label>
                <input
                  type="text"
                  placeholder="/usr/bin/app or https://example.com"
                  value={formData.target}
                  onChange={(e) => setFormData({...formData, target: e.target.value})}
                  required
                />
              </div>

              <div className="form-group">
                <label>Icon (Optional)</label>
                <input
                  type="file"
                  accept="image/*"
                  onChange={handleIconUpload}
                />
                {formData.icon && (
                  <div className="icon-preview">
                    <img src={formData.icon} alt="Preview" />
                  </div>
                )}
              </div>

              <div className="modal-actions">
                <button type="submit" className="btn-submit">
                  {selectedLauncher ? 'Update' : 'Create'} Launcher
                </button>
                <button 
                  type="button"
                  className="btn-cancel"
                  onClick={closeModal}
                >
                  Cancel
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  )
}

export default App