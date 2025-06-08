# 🚀 Caddy Manager

A beautiful, modern web interface for managing Caddy reverse proxy configurations. Built with Vue.js 3, TypeScript, and PrimeVue components.

![Caddy Manager Dashboard](https://via.placeholder.com/800x400?text=Caddy+Manager+Dashboard)

## ✨ Features

### 🎯 Core Functionality

- **Beautiful Dashboard** - Real-time system overview with statistics and metrics
- **Host Management** - Add, edit, and delete backend servers with ease
- **Modern UI** - Clean, responsive design with PrimeVue Aura theme
- **Real-time Updates** - Live data refresh and system monitoring
- **Mobile-Friendly** - Fully responsive design for all devices

### 🎨 UI/UX Features

- **Professional Navigation** - Collapsible sidebar with smooth animations
- **Interactive Charts** - Visual statistics and performance metrics
- **Advanced Search** - Filter and search through hosts and configurations
- **Toast Notifications** - Real-time feedback for user actions
- **Confirmation Dialogs** - Safe deletion with user confirmation
- **Loading States** - Beautiful loading indicators and transitions

### 🔧 Technical Features

- **Vue.js 3** with Composition API
- **TypeScript** for type safety
- **PrimeVue** components with Aura theme
- **Chart.js** integration for data visualization
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

1. **Clone the repository**

   ```bash
   git clone <repository-url>
   cd caddymanager
   ```

2. **Install frontend dependencies**

   ```bash
   cd frontend/caddymanager-ui
   npm install
   ```

3. **Start the development server**

   ```bash
   npm run dev
   ```

4. **Start the backend server** (in a separate terminal)

   ```bash
   cd backend
   cargo run
   ```

5. **Open your browser**
   Navigate to `http://localhost:5173` to access the beautiful UI!

### Backend API

The backend runs on `http://localhost:8080` and provides:

- `GET /api/hosts` - List all hosts
- `POST /api/hosts` - Add a new host
- `DELETE /api/hosts/{name}` - Delete a host

## 🎨 Design System

### Color Palette

- **Primary**: PrimeVue Aura theme colors
- **Success**: Green tones for active/online states
- **Warning**: Orange tones for maintenance/warnings
- **Danger**: Red tones for errors/offline states
- **Info**: Blue tones for informational content

### Typography

- **Headings**: Bold, modern font weights
- **Body**: Clean, readable text with proper contrast
- **Code**: Monospace font for technical content

### Components

- **Cards**: Elevated surfaces with subtle shadows
- **Buttons**: Consistent styling with hover states
- **Forms**: Clear labels with validation feedback
- **Tables**: Sortable, searchable data grids
- **Navigation**: Smooth transitions and active states

## 📱 Responsive Design

The application is fully responsive with breakpoints:

- **Desktop**: 1024px+ (full sidebar, grid layouts)
- **Tablet**: 768px - 1023px (collapsible sidebar)
- **Mobile**: < 768px (mobile-optimized navigation)

## 🔧 Development

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run type-check` - Run TypeScript checks
- `npm run lint` - Run ESLint
- `npm run format` - Format code with Prettier

### Development Guidelines

- Use TypeScript for all new components
- Follow Vue.js 3 Composition API patterns
- Maintain responsive design principles
- Include proper loading and error states
- Add appropriate animations and transitions

## 🚀 Production Deployment

1. **Build the frontend**

   ```bash
   npm run build
   ```

2. **Build the backend**

   ```bash
   cargo build --release
   ```

3. **Deploy both applications**
   - Frontend: Serve the `dist/` folder with a web server
   - Backend: Run the compiled binary with Caddy

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes with proper styling
4. Test responsiveness on all device sizes
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## 🙋‍♂️ Support

For questions or support, please open an issue on GitHub.

---

Made with ❤️ using Vue.js 3, TypeScript, and PrimeVue
