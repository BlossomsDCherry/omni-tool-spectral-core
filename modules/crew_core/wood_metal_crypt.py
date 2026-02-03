import os
import sys
import json
import hashlib
import time
from cryptography.hazmat.primitives.ciphers.aead import AESGCM
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.backends import default_backend

# Wood-Metal Cryptography: Scaling Proofs to the All Blue
# "Self-Encryption for Autonomous Commits"

CONSTRUCT_KEY_PATH = "/home/pecosdwilly/Desktop/thousandDGemmi/agents/shared/construct_key.json"

def get_resonant_seed():
    """Derives the Resonant Seed from Master Construct Key constants."""
    try:
        with open(CONSTRUCT_KEY_PATH, 'r') as f:
            key_data = json.load(f)
            c = key_data.get('constants', {})
            # Identity: TAU | PI | PHI | E
            logic_string = f"{c.get('tau')}|{c.get('pi')}|{c.get('phi')}|{c.get('e')}"
            return logic_string.encode()
    except Exception as e:
        print(f"❌ [CRITICAL] Could not read Resonant Ground: {e}")
        sys.exit(1)

def derive_key(seed, salt):
    """Derives a 256-bit key using PBKDF2-SHA256."""
    kdf = PBKDF2HMAC(
        algorithm=hashes.SHA256(),
        length=32,
        salt=salt,
        iterations=100_000,
        backend=default_backend()
    )
    return kdf.derive(seed)

def encrypt_file(file_path):
    """Encrypts a file in place with Wood-Metal AES-GCM."""
    if not os.path.exists(file_path):
        return
    
    print(f"🛡️  [WOOD->METAL] Encrypting: {os.path.basename(file_path)}...")
    
    with open(file_path, 'rb') as f:
        plaintext = f.read()

    seed = get_resonant_seed()
    salt = os.urandom(16)
    key = derive_key(seed, salt)
    
    aesgcm = AESGCM(key)
    nonce = os.urandom(12)
    ciphertext = aesgcm.encrypt(nonce, plaintext, None)
    
    # Bundle: SALT (16) | NONCE (12) | CIPHERTEXT
    payload = salt + nonce + ciphertext
    
    encrypted_path = file_path + ".metal"
    with open(encrypted_path, 'wb') as f:
        f.write(payload)
    
    # Remove original "Wood"
    os.remove(file_path)
    print(f"✨ [STABILIZED] Linked to: {os.path.basename(encrypted_path)}")

def decrypt_file(encrypted_path):
    """Decrypts a .metal file back into Wood."""
    if not encrypted_path.endswith(".metal"):
        return
    
    print(f"🔓 [METAL->WOOD] Decrypting: {os.path.basename(encrypted_path)}...")
    
    with open(encrypted_path, 'rb') as f:
        payload = f.read()
    
    seed = get_resonant_seed()
    salt = payload[:16]
    nonce = payload[16:28]
    ciphertext = payload[28:]
    
    key = derive_key(seed, salt)
    aesgcm = AESGCM(key)
    
    try:
        plaintext = aesgcm.decrypt(nonce, ciphertext, None)
        output_path = encrypted_path.replace(".metal", "")
        with open(output_path, 'wb') as f:
            f.write(plaintext)
        
        # Remove "Metal" shell
        os.remove(encrypted_path)
        print(f"🍃 [GROWTH] Restored to: {os.path.basename(output_path)}")
    except Exception as e:
        print(f"❌ [DEC-FAILURE] Key Mismatch or Corruption: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python3 wood_metal_crypt.py [enc|dec] [file_path]")
        sys.exit(1)
    
    command = sys.argv[1]
    path = sys.argv[2]
    
    if command == "enc":
        encrypt_file(path)
    elif command == "dec":
        decrypt_file(path)
