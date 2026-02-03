import os
import datetime
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from sklearn.cluster import KMeans

# Configuration
METADATA_FILE = "/media/pecosdwilly/AceOhara/Ship Manifests/Current/sovereign_metadata.txt"
OUTPUT_IMAGE = "sovereign_cross_chart.png"
OUTPUT_LOG = "forensic_log.txt"

def parse_permissions_score(perm_str):
    """
    Calculates Substrate Score (Stability) based on Permissions.
    Strict = High Substrate (1.0). Loose = High Void (0.0).
    
    Heuristic: Sum of 'rwx' bits.
    r=4, w=2, x=1.
    Owner, Group, Other.
    Max looseness = 7+7+7 = 21.
    """
    # Skip type indicator (d/-)
    perms = perm_str[1:]
    score = 0
    # Simple rigorous mapping
    # r, w, x occur at indices? No, just iterate.
    # actually, standard rwx r-x r-- format.
    # 9 chars.
    
    current_val = 0
    for i, char in enumerate(perms):
        # Weight doesn't matter as much as total "openness" count
        # But 'w' is more "void-inducing" (changeable) than 'r'
        if char == 'r': current_val += 1
        elif char == 'w': current_val += 2
        elif char == 'x': current_val += 1 # exec is functional, neutral-ish?
    
    # Max possible value? 
    # rwx = 4+2+1? No, let's use standard integer.
    # r=4, w=2, x=1
    
    # Explicit loop for standard Octal sum
    # u
    u = (4 if perms[0]=='r' else 0) + (2 if perms[1]=='w' else 0) + (1 if perms[2]=='x' else 0)
    # g
    g = (4 if perms[3]=='r' else 0) + (2 if perms[4]=='w' else 0) + (1 if perms[5]=='x' else 0)
    # o
    o = (4 if perms[6]=='r' else 0) + (2 if perms[7]=='w' else 0) + (1 if perms[8]=='x' else 0)
    
    total_loose = u + g + o 
    # Max is 21 (777)
    
    # Normalized Looseness (Void Potential)
    void_potential = total_loose / 21.0
    
    # Substrate Integrity = 1.0 - Void Potential
    return 1.0 - void_potential

def parse_date(date_str):
    try:
        main_part = date_str.split(' +')[0]
        if '.' in main_part:
            time_part = main_part.split('.')
            main_part = time_part[0]
            micros = time_part[1][:6]
            dt_str = f"{main_part}.{micros}"
            dt = datetime.datetime.strptime(dt_str, "%Y-%m-%d %H:%M:%S.%f")
        else:
            dt = datetime.datetime.strptime(main_part, "%Y-%m-%d %H:%M:%S")
        return dt.timestamp()
    except Exception as e:
        return 0.0

def load_data(filepath):
    files = []
    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()
            if not line: continue
            parts = line.split(' | ')
            if len(parts) < 4: continue
            
            perms_str = parts[0]
            created_str = parts[1].replace("Created: ", "")
            modified_str = parts[2].replace("Modified: ", "")
            filename = parts[3]
            
            p_score = parse_permissions_score(perms_str)
            c_ts = parse_date(created_str)
            m_ts = parse_date(modified_str)
            
            files.append({
                'name': filename,
                'p_score': p_score, # Substrate Integrity
                'c_ts': c_ts,
                'm_ts': m_ts
            })
    return files

def main():
    print("Initializing Forensic Navigator (Three Cross Two)...")
    data = load_data(METADATA_FILE)
    n = len(data)
    
    if n == 0:
        print("No data.")
        return

    # Vectors
    names = [d['name'] for d in data]
    substrate_vec = np.array([d['p_score'] for d in data], dtype=float)
    
    # Time (Recency)
    # We want "Time" on an axis. 
    # Let's say Normalized Mod Time (0=Oldest, 1=Newest)
    m_vec = np.array([d['m_ts'] for d in data], dtype=float)
    m_min, m_max = np.min(m_vec), np.max(m_vec)
    if m_max > m_min:
        time_vec = (m_vec - m_min) / (m_max - m_min)
    else:
        time_vec = np.zeros_like(m_vec)
        
    # Logic: Gravity (Void Intensity)
    # Users Logic: "Strict Permissions... Stable". 
    # So Gravity (Instability) increases with Looseness (1-Substrate) and Recency (Active).
    # Simple model: Gravity = (1 - Substrate) * Time_Activity
    # Note: If something is OLD and LOOSE, is it Void? Yes, an abandoned well.
    # If something is NEW and STRICT, is it Void? Less so, it's a hardened new feature.
    # Let's define Gravity = (Looseness + Recency) / 2
    looseness = 1.0 - substrate_vec
    gravity_vec = (looseness + time_vec) / 2.0
    
    # Dimensions for Plotting
    X = substrate_vec  # Substrate Integrity
    Y = time_vec       # Temporal Resonance (Newness)
    Z = gravity_vec    # Void Intensity
    
    # Octant Clustering (K=8)
    # We cluster on these 3 dimensions
    features = np.column_stack((X, Y, Z))
    kmeans = KMeans(n_clusters=8, random_state=42)
    labels = kmeans.fit_predict(features)
    
    # Analysis per Octant
    octant_stats = []
    print("\n--- Octant Analysis (The 8 Gates) ---")
    log_file = open(OUTPUT_LOG, 'w')
    log_file.write("--- Forensic Navigation Log ---\n")
    log_file.write("Methodology: Three Cross Two (Substrate vs Void)\n\n")

    for i in range(8):
        mask = (labels == i)
        count = np.sum(mask)
        if count == 0: continue
        
        avg_stab = np.mean(substrate_vec[mask])
        avg_grav = np.mean(gravity_vec[mask])
        members = [names[j] for j, is_in in enumerate(mask) if is_in]
        
        # Center of Mass
        center = kmeans.cluster_centers_[i]
        
        info = f"Octant {i}: N={count} | Substrate: {avg_stab:.2f} | Gravity: {avg_grav:.2f}"
        print(info)
        log_file.write(info + "\n")
        log_file.write(f"  > Anchor Files: {members[:3]}...\n\n")
        
        octant_stats.append({
            'id': i,
            'gravity': avg_grav,
            'substrate': avg_stab
        })
        
    # Identify Harmonic Exit (Max Substrate) & Event Horizon (Max Gravity)
    best_octant = max(octant_stats, key=lambda x: x['substrate'])
    worst_octant = max(octant_stats, key=lambda x: x['gravity'])
    
    log_file.write(f"\n[RECOMMENDATION]\n")
    log_file.write(f"Harmonic Exit (Safe Anchor): Octant {best_octant['id']} (Substrate: {best_octant['substrate']:.2f})\n")
    log_file.write(f"Event Horizon (Active Void): Octant {worst_octant['id']} (Gravity: {worst_octant['gravity']:.2f})\n")
    log_file.close()

    # Visualization: 3D Cross Chart
    fig = plt.figure(figsize=(12, 10))
    ax = fig.add_subplot(111, projection='3d')
    
    # Scatter plot formatted as terrain points
    # Color by Octant (Cluster) or Gravity?
    # Let's Color by Gravity to show the "Field" flow
    img = ax.scatter(X, Y, Z, c=Z, cmap='magma', s=60, edgecolors='black')
    
    ax.set_xlabel('Substrate Integrity (Strictness)')
    ax.set_ylabel('Temporal Resonance (Recency)')
    ax.set_zlabel('Void Intensity (Gravity)')
    ax.set_title('Sovereign Cross Chart: (3x2)x2 Topology')
    
    # Add Colorbar
    cbar = fig.colorbar(img, ax=ax, pad=0.1)
    cbar.set_label('Gravity (Void)')
    
    # Annotate significant points (Outliers or Center of Gates)
    # Annotate Octant Centers with Number
    centers = kmeans.cluster_centers_
    for i, c in enumerate(centers):
        ax.text(c[0], c[1], c[2], f"Gate {i}", fontsize=12, fontweight='bold', color='blue')

    plt.savefig(OUTPUT_IMAGE, dpi=300)
    print(f"Cross Chart saved to {os.path.abspath(OUTPUT_IMAGE)}")
    print(f"Log saved to {os.path.abspath(OUTPUT_LOG)}")

if __name__ == "__main__":
    main()
