import json
import os

# Папка, где лежит скрипт (и где должен лежать smdData.json)
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
data_path = os.path.join(SCRIPT_DIR, 'smdData.json')

def count_entries(path):
    if not os.path.exists(path):
        print(f"❌ Файл не найден: {path}")
        print("   Положи 'smdData.json' в ту же папку, где лежит этот скрипт.")
        return

    print(f"Читаю {path}... (это может занять пару секунд)")
    try:
        with open(path, 'r', encoding='utf-8') as f:
            data = json.load(f)
    except Exception as e:
        print(f"❌ Ошибка чтения JSON: {e}")
        return

    total = len(data)
    
    # Считаем уникальные коды
    unique_codes = set()
    for entry in data:
        code = entry.get("code")
        if code:
            unique_codes.add(code)

    print("\n" + "="*40)
    print(f"📊 РЕЗУЛЬТАТ:")
    print(f"   Всего записей (деталей): {total:,}".replace(',', ' '))
    print(f"   Уникальных кодов:        {len(unique_codes):,}".replace(',', ' '))
    print("="*40 + "\n")

if __name__ == '__main__':
    count_entries(data_path)