import { NextRequest, NextResponse } from 'next/server'

// Interface for wave code configuration
interface WaveCodeConfig {
    title: string
    columns: number
    backgroundColor: string
    imageSize: number
}

// Generate a single song div with Spotify wave code
function generateSongDiv(trackId: string, imageSize: number = 640): string {
    const spotifyCodeUrl = `https://scannables.scdn.co/uri/plain/png/000000/white/${imageSize}/spotify:track:${trackId}`
    return `    <div class="song">
        <img src="${spotifyCodeUrl}" alt="Spotify Code">
    </div>`
}

// Generate CSS styles for the wave codes page
function generateCSS(config: WaveCodeConfig): string {
    return `        body {
            font-family: Arial, sans-serif;
            margin: 10px;
            padding: 0;
            background-color: ${config.backgroundColor};
            display: grid;
            grid-template-columns: repeat(${config.columns}, 1fr);
            column-gap: 1px;
            row-gap: 1px;
        }
        .song {
            margin: 0;
            padding: 0;
            box-shadow: none;
            border-radius: 0;
            text-align: center;
            page-break-inside: avoid;
        }
        img {
            max-width: 100%;
            height: auto;
            border: none;
            border-radius: 0;
            display: block;
        }
        @media print {
            body { padding: 0; margin: 0; background: white; }
            .song { margin: 0; box-shadow: none; border: none; }
            @page {
                margin: 10px;
            }
        }`
}

// Generate complete HTML page with Spotify wave codes
function generateWaveCodesPage(trackIds: string[], config: WaveCodeConfig): string {
    // Generate all song divs
    const songDivs = trackIds.map(trackId => generateSongDiv(trackId, config.imageSize))
    const songsHtml = songDivs.join('\\n')

    // Generate CSS
    const css = generateCSS(config)

    // Complete HTML template
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${config.title}</title>
    <style>
${css}
    </style>
</head>
<body>
${songsHtml}
</body>
</html>`
}

export async function POST(request: NextRequest) {
    try {
        const { trackIds, title, columns, imageSize, backgroundColor } = await request.json()

        if (!trackIds || !Array.isArray(trackIds) || trackIds.length === 0) {
            return NextResponse.json({ success: false, error: 'Track IDs are required' })
        }

        // Create configuration
        const config: WaveCodeConfig = {
            title: title || 'Spotify Codes Printable Page',
            columns: columns || 4,
            backgroundColor: backgroundColor || 'white',
            imageSize: imageSize || 640
        }

        // Generate HTML
        const html = generateWaveCodesPage(trackIds, config)

        return NextResponse.json({
            success: true,
            html,
            message: `Generated HTML with ${trackIds.length} tracks in ${config.columns}-column layout`
        })
    } catch (error) {
        return NextResponse.json({
            success: false,
            error: `Server error: ${error}`
        })
    }
}
