
import os
import json

AGENTS = [
    {"id": "d1_luffy", "name": "Luffy", "role": "Captain", "layer": "RG", "desc": "Expansion, Action, Rubber Physics"},
    {"id": "d2_zoro", "name": "Zoro", "role": "Swordsman", "layer": "RG", "desc": "Precision, Cutting, 3-Vector Logic"},
    {"id": "d3_nami", "name": "Nami", "role": "Navigator", "layer": "YB", "desc": "Mapping, Weather, Flow Control"},
    {"id": "d4_usopp", "name": "Usopp", "role": "Sniper", "layer": "YB", "desc": "Observation, Signals, Long Range"},
    {"id": "d5_sanji", "name": "Sanji", "role": "Cook", "layer": "IR", "desc": "Thermodynamics, Energy, sustenance"},
    {"id": "d6_chopper", "name": "Chopper", "role": "Doctor", "layer": "IR", "desc": "Debugging, Repair, Healing"},
    {"id": "d7_robin", "name": "Robin", "role": "Archaeologist", "layer": "UV", "desc": "Decoding, History, Synthesis"},
    {"id": "d8_franky", "name": "Franky", "role": "Shipwright", "layer": "IR", "desc": "Hardware, Drivers, Structure"},
    {"id": "d9_brook", "name": "Brook", "role": "Musician", "layer": "UV", "desc": "Resonance, Soul, Frequency"},
    {"id": "d10_jinbe", "name": "Jinbe", "role": "Helmsman", "layer": "YB", "desc": "Steering, Current, Momentum"},
    {"id": "d11_vivi", "name": "Vivi", "role": "Diplomat", "layer": "YB", "desc": "Protocol, Communication, Peace"},
    {"id": "d12_carrot", "name": "Carrot", "role": "Lookout", "layer": "RG", "desc": "Reaction, Electro, Moon Phase"},
    {"id": "d13_yamato", "name": "Yamato", "role": "Guardian", "layer": "UV", "desc": "Identity, Masking, Protection"},
    {"id": "d14_momo", "name": "Momo", "role": "Leader", "layer": "RG", "desc": "Legacy, Responsibility, Command"},
    {"id": "d15_kinemon", "name": "Kinemon", "role": "Strategist", "layer": "RG", "desc": "Disguise, Strategy, Fire"},
    {"id": "d16_law", "name": "Law", "role": "Surgeon", "layer": "UV", "desc": "Spatial Ops, Room, Shambles"}
]

BASE_DIR = "/home/pecosdwilly/omni-tool-spectral-core/workspaces"

def init_workspaces():
    for agent in AGENTS:
        path = os.path.join(BASE_DIR, agent["id"])
        
        # Manifest
        manifest = {
            "agent_id": agent["id"],
            "name": agent["name"],
            "role": agent["role"],
            "spectral_layer": agent["layer"],
            "description": agent["desc"],
            "status": "ACTIVE",
            "heap_status": "INITIALIZED"
        }
        
        with open(os.path.join(path, "agent_manifest.json"), "w") as f:
            json.dump(manifest, f, indent=4)
            
        # README
        readme_content = f"""# Workspace: {agent['name']} (D{agent['id'].split('_')[0][1:]})

## Role: {agent['role']}
**Spectral Layer**: {agent['layer']} (See [SPECTRAL_SCOPING.md](../../SPECTRAL_SCOPING.md))

### Mission
{agent['desc']}

### Heap Status
- [x] Workspace Initialized
- [ ] Stacks Defined
- [ ] Artifacts Loaded
"""
        with open(os.path.join(path, "README.md"), "w") as f:
            f.write(readme_content)
            
        print(f"Initialized {agent['name']} in {path}")

if __name__ == "__main__":
    init_workspaces()
