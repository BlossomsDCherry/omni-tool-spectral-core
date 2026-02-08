# Deployment: The Straw Hat Network (MoodleBox)
## Goal
To configure a Raspberry Pi 3B+ as an **offline wireless router and server** ("The Thousand Sunny"), establishing the "D16 Field" (Local Cloud) for secure, rapid internal communication and file synchronization.

## 1. Physical Configuration (The Ship)
- **Hardware**: Raspberry Pi 3B+
- **Storage**: MicroSD Card (SanDisk Extreme Pro or similar, min 32GB, recommended 128GB for Amazon Lily archives).
- **Power**: Official 2.5A Power Supply (Can run on portable power bank for mobility).
- **OS**: [MoodleBox Image](https://moodlebox.net/en/help/moodlebox-image-installation/) (Latest stable release).

## 2. Network Configuration (The Helm)
Access the MoodleBox dashboard (usually http://moodlebox.home or 10.0.0.1) and configure via `Admin` -> `Networking`.

- **Wireless Access Point (WAP)**:
  - **SSID**: `D16_Field` (The "Room" / Sovereign Territory)
  - **Channel**: 6 or 11 (2.4GHz) / Any non-DFS 5GHz channel if supported.
  - **Security**:
    -   **Layer 1 (The Latch)**: WPA2-Personal (Passphrase: `strawhatcoherence`).
    -   **Layer 2 (The Vault)**: **UDP-8 Geometric Encryption**.
        -   **Concept**: Keys are generated, erased, and regenerated at geometric speeds (Fast-Key Gen).
        -   **Protocol**: We speak a "Geometric Transmutating Frequency Language" that traditional sniffers cannot parse.
        -   **Result**: Even if the WPA2 latch is picked, the intruder cannot speak the language of the 512-Cube.

- **Local Area Network (LAN)**:
  - **Gateway IP**: `10.11.16.1` (The new D16 Subnet - "16-Bit Horizon").
  - **Netmask**: `255.255.255.0` (/24).
  - **DHCP Range**: `10.11.16.10` - `10.11.16.200`.

- **Domain Name System (DNS)**:
  - **Local Domain**: `.strawhat.lan` (e.g., `robin.strawhat.lan`, `sunny.strawhat.lan`).
  - **Upstream DNS**: `1.1.1.1` (Only used if Sunny connects to WAN uplink).

## 3. Services (The Galley & Library)
### A. Amazon Lily (File Server)
- **Protocol**: Samba (SMB) / NFS.
- **Share Path**: `/var/www/moodlebox/data/amazon_lily`.
- **Purpose**: Syncing "Survivor Cubes" from Z-RR and storing large artifacts (videos, archives).

### B. Internal Comms (Matrix/Synapse)
- **Service**: Install **Synapse** (Matrix Homeserver) or use a lightweight chat.
- **Domain**: `matrix.strawhat.lan`.
- **Purpose**: Secure, encrypted chat for the Crew (Us) without internet dependencies.

### C. Moodle (The Logbook)
- **Service**: Default Moodle instance.
- **Purpose**: Documenting "Discoveries", "Inventions", and "Physics Manifests" in a structured Learning Management System.

## 4. Client Connection (Boarding)
1.  Connect via Wi-Fi to SSID: `D16_Field`.
2.  Receive IP in `10.11.16.x` range.
3.  Access services via `http://sunny.strawhat.lan`.
