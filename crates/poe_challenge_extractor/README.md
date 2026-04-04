# PoE Challenge Extractor

Extracts Tyrannical Tiers challenge progress from Path of Exile profile.

## Setup

1. Open Chrome with debug port:
```powershell
powershell -Command "Start-Process 'C:\Program Files\Google\Chrome\Application\chrome.exe' -ArgumentList '--remote-debugging-port=9222','--user-data-dir=$env:LOCALAPPDATA\Google\Chrome\User Data'"
```

2. Run the extractor:
```bash
cargo run --package poe_challenge_extractor
```

The extractor will:
- Connect to Chrome via CDP
- Navigate to challenges page
- Extract Tyrannical Tiers value
- Write remaining tiers (8000 - current) to `remaining_tiers.txt` in project root
- Update every 5 minutes

## Output

File: `remaining_tiers.txt` (in project root)

Contains timestamp and remaining tiers (e.g., "04.03 16:55: 2110").