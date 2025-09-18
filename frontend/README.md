# Wave Code Generator Frontend

A beautiful Next.js frontend for generating Spotify wave codes with customizable options.

## Features

🎨 **Beautiful UI**: Modern, responsive design with Tailwind CSS
🎵 **Spotify Integration**: Extract tracks directly from playlist URLs
⚙️ **Customizable**: Adjust columns, colors, image sizes, and more
📱 **Responsive**: Works great on desktop and mobile
⚡ **Fast**: Built with Next.js and TypeScript

## Setup

### 1. Install Dependencies

```bash
npm install
```

### 2. Configure Spotify API

1. Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard/)
2. Create a new app
3. Copy your Client ID and Client Secret
4. Create `.env.local` file:

```bash
cp env.example .env.local
```

5. Edit `.env.local` with your credentials:

```
SPOTIFY_CLIENT_ID=your_client_id_here
SPOTIFY_CLIENT_SECRET=your_client_secret_here
```

### 3. Build Rust Backend

Make sure the Rust backend is built:

```bash
cd .. && cargo build
```

### 4. Run Development Server

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the app.

## How to Use

1. **Paste Spotify Playlist URL**: Copy any Spotify playlist URL
2. **Extract Track IDs**: Click "Extract Track IDs" to get all songs
3. **Customize Settings**: 
   - Adjust number of columns (1-8)
   - Change image size (320px-1280px)
   - Pick background color
   - Set custom title
4. **Generate HTML**: Click "Generate Wave Codes" to create the HTML
5. **Download**: Preview and download your wave code page

## Customization Options

- **Columns**: 1-8 columns for different layouts
- **Image Size**: 320px to 1280px for different quality/size needs
- **Background Color**: Any hex color or color picker
- **Page Title**: Custom title for your wave code page

## API Endpoints

- `POST /api/extract-tracks`: Extract track IDs from Spotify playlist URL
- `POST /api/generate-html`: Generate HTML with customizable wave codes

## Tech Stack

- **Next.js 14**: React framework with App Router
- **TypeScript**: Type safety
- **Tailwind CSS**: Utility-first styling
- **Lucide React**: Beautiful icons
- **Headless UI**: Accessible components

## Project Structure

```
frontend/
├── src/
│   ├── app/
│   │   ├── api/           # API routes
│   │   ├── page.tsx       # Main UI
│   │   └── globals.css    # Global styles
│   └── ...
├── env.example            # Environment variables template
└── README.md
```

## Development

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

## Integration with Rust Backend

The frontend communicates with the Rust backend through:

1. **Track Extraction**: Calls `get-song-ids` binary via Node.js child process
2. **HTML Generation**: Uses TypeScript implementation of the Rust wave code generator
3. **File Management**: Reads/writes to the `input/` and `output/` folders

The Rust backend must be built (`cargo build`) for the frontend to work properly.