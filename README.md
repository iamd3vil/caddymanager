# 🚀 Caddy Manager

A beautiful, modern web interface for managing Caddy reverse proxy configurations. Built with Vue.js 3, TypeScript, and PrimeVue components.

![Caddy Manager Dashboard](https://via.placeholder.com/800x400?text=Caddy+Manager+Dashboard)

## ✨ Features

### 🎯 Core Functionality

- **Host Management** - Add, edit, and delete backend servers with ease

### 🔧 Technical Features

- **Vue.js 3** with Composition API
- **TypeScript** for type safety
- **PrimeVue** components with Aura theme
- **Responsive Design** with CSS Grid and Flexbox
- **Modern Styling** with CSS custom properties

## 🏗️ Architecture

```
frontend/caddymanager-ui/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── AppHeader.vue   # Main header with navigation
│   │   └── AppSidebar.vue  # Collapsible sidebar menu
│   ├── views/              # Page components
│   │   ├── DashboardView.vue  # Main dashboard with stats
│   │   ├── HostsView.vue      # Host management page
│   │   ├── ConfigView.vue     # Configuration page (coming soon)
│   │   ├── LogsView.vue       # Logs viewer (coming soon)
│   │   └── StatsView.vue      # Advanced statistics (coming soon)
│   ├── router/             # Vue Router configuration
│   ├── assets/            # Stylesheets and static assets
│   ├── App.vue           # Root component with layout
│   └── main.ts          # Application entry point
└── package.json
```

## 🚀 Getting Started

### Prerequisites

- Node.js 18+ and npm
- Rust and Cargo (for backend)

### Installation

[[TBD]]

### Backend API

The backend runs on `http://localhost:8080` and provides:

- `GET /api/hosts` - List all hosts
- `POST /api/hosts` - Add a new host
- `DELETE /api/hosts/{name}` - Delete a host

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes with proper styling
4. Test responsiveness on all device sizes
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.
