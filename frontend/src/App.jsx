import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

function App() {
  const [launchers, setLaunchers] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState(null)
  const [formData, setFormData] = useState({
    id: '',
    name: '',
    type: 'app',
    target: ''
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
      await invoke('add_launcher_cmd', {
        id: formData.id,
        name: formData.name,
        launchType: formData.type,
        target: formData.target,
      })
      setFormData({ id: '', name: '', type: 'app', target: '' })
      await loadLaunchers()
    } catch (err) {
      setError(err.toString())
    }
  }

  async function removeLauncher(id) {
    try {
      await invoke('remove_launcher_cmd', { id })
      await loadLaunchers()
    } catch (err) {
      setError(err.toString())
    }
  }

  async function executeLauncher(id) {
    try {
      const result = await invoke('execute_launcher_cmd', { id })
      alert(result)
    } catch (err) {
      alert('Error: ' + err.toString())
    }
  }

  return (
    <div className="app">
      <header className="header">
        <h1>🚀 Initium Dashboard</h1>
        <p>Manage your launchers</p>
      </header>

      {error && (
        <div className="error">
          ❌ {error}
          <button onClick={() => setError(null)}>✕</button>
        </div>
      )}

      <main className="container">
        {/* Add Launcher Form */}
        <section className="form-section">
          <h2>Add New Launcher</h2>
          <form onSubmit={addLauncher} className="form">
            <input
              type="text"
              placeholder="ID"
              value={formData.id}
              onChange={(e) => setFormData({...formData, id: e.target.value})}
              required
            />
            <input
              type="text"
              placeholder="Name"
              value={formData.name}
              onChange={(e) => setFormData({...formData, name: e.target.value})}
              required
            />
            <select
              value={formData.type}
              onChange={(e) => setFormData({...formData, type: e.target.value})}
            >
              <option value="app">App</option>
              <option value="web">Web</option>
            </select>
            <input
              type="text"
              placeholder="Target (path or URL)"
              value={formData.target}
              onChange={(e) => setFormData({...formData, target: e.target.value})}
              required
            />
            <button type="submit">Add Launcher</button>
          </form>
        </section>

        {/* Launchers List */}
        <section className="launchers-section">
          <h2>Launchers ({launchers.length})</h2>
          {loading ? (
            <p>Loading...</p>
          ) : launchers.length === 0 ? (
            <p>No launchers yet. Add one above!</p>
          ) : (
            <div className="launchers-grid">
              {launchers.map((launcher) => (
                <div key={launcher.id} className="launcher-card">
                  <div className="launcher-info">
                    <h3>{launcher.name}</h3>
                    <p className="type">{launcher.launch_type.toUpperCase()}</p>
                    <p className="target">{launcher.target}</p>
                  </div>
                  <div className="launcher-actions">
                    <button 
                      className="btn-execute"
                      onClick={() => executeLauncher(launcher.id)}
                    >
                      ▶ Execute
                    </button>
                    <button 
                      className="btn-delete"
                      onClick={() => removeLauncher(launcher.id)}
                    >
                      🗑 Delete
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>
      </main>
    </div>
  )
}

export default App