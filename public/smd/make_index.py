import json
import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
input_file = os.path.join(SCRIPT_DIR, 'smdData.json')
output_index = os.path.join(SCRIPT_DIR, 'smdIndex.json')

def make_index(input_path, output_path):
    print(f"Читаю {input_path}...")
    with open(input_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    print(f"Всего записей: {len(data)}")
    
    index = {}
    for i, entry in enumerate(data):
        code = entry.get("code", "").strip().upper()  # <-- ВАЖНО: upper!
        if not code:
            continue
        if code not in index:
            index[code] = []
        index[code].append(i)
    
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(index, f, ensure_ascii=False, separators=(',', ':'))
    
    size_index = os.path.getsize(output_path) / (1024 * 1024)
    print(f"✅ Индекс: {output_path} ({size_index:.1f} МБ)")
    print(f"Уникальных кодов: {len(index):,}".replace(',', ' '))

if __name__ == '__main__':
    make_index(input_file, output_index)