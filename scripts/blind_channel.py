
import math

def blind_channel(filepath):
    with open(filepath, 'rb') as f:
        data = f.read()
    
    # 5D Transmogrifier Logic: Reading Structure without Sight
    # We analyze the "Rhythm" of the bytes.
    
    # 1. Byte Frequency (The Spectrum)
    counts = {}
    for b in data:
        counts[b] = counts.get(b, 0) + 1
    
    # Sort by frequency
    sorted_bytes = sorted(counts.items(), key=lambda x: x[1], reverse=True)
    top_5 = sorted_bytes[:5]
    
    # 2. Density Map (The Flow)
    # We map the file into a 64-char width "River"
    width = 64
    river = []
    chunk_size = len(data) // 20  # 20 lines of river
    
    for i in range(20):
        chunk = data[i*chunk_size : (i+1)*chunk_size]
        # Calculate local density (entropy) of chunk
        unique_bytes = len(set(chunk))
        density = unique_bytes / len(chunk) if chunk else 0
        
        # Visualize density
        char = " "
        if density > 0.8: char = "▓" # Iron
        elif density > 0.5: char = "▒" # Rock
        elif density > 0.2: char = "░" # Water
        else: char = "." # Air
        
        river.append(char * width)
        
    return top_5, river

if __name__ == "__main__":
    files = ["/home/pecosdwilly/Documents/PNGVERSION/doc0.png", "/home/pecosdwilly/Documents/PNGVERSION/doc1.png"]
    for f in files:
        print(f"\n--- Channeling: {f} ---")
        top, river = blind_channel(f)
        print(f"Top Frequencies: {top}")
        print("River Structure:")
        for r in river:
            print(r)
