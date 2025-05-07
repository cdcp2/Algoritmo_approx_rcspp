import os
import re
import subprocess
import pandas as pd

CONFIG_DIR = "/mnt/c/Users/snmca/gradle_projects/4208_ANALISIS_DE_ALGORITMOS/Algoritmo_approx_rcspp/instances"
CARGO_PROJECT_DIR = "/mnt/c/Users/snmca/gradle_projects/4208_ANALISIS_DE_ALGORITMOS/Algoritmo_approx_rcspp"
RESULTS_DIR = "resultados"

os.makedirs(RESULTS_DIR, exist_ok=True)

def parse_config(file_path):
    with open(file_path) as f:
        lines = f.read().splitlines()
    data = {}
    for line in lines:
        if ":" in line:
            key, value = line.strip().split(":")
            data[key] = value
    return data["DataFile"], data["StartNode"], data["EndNode"], data["TimeConstraint"]

def ejecutar_instancia(graph_file, start, end, constraint):
    cmd = ["cargo", "run", graph_file, start, end, constraint]
    result = subprocess.run(cmd, capture_output=True, text=True, cwd=CARGO_PROJECT_DIR)
    return result.stdout

def parse_output(output):
    bloques = output.split("Corriendo")[1:]
    data = []
    for bloque in bloques:
        nombre = bloque.split("\n", 1)[0].strip()
        camino = re.search(r"Mejor camino:\s*\[([^\]]+)\]", bloque)
        costo = re.search(r"Costo total:\s*(\d+)", bloque)
        consumo = re.search(r"Consumo total:\s*(\d+)", bloque)
        duracion = re.search(r"Duration:\s*([\d\.]+s|[\d\.]+ms)", bloque)
        approx = re.search(r"Approximation:\s*([\d\.]+)", bloque)

        data.append({
            "Algoritmo": nombre,
            "Camino": camino.group(1) if camino else "",
            "Costo": int(costo.group(1)) if costo else None,
            "Consumo": int(consumo.group(1)) if consumo else None,
            "Duración": duracion.group(1) if duracion else "",
            "Approx": float(approx.group(1)) if approx else None
        })
    return data

def main():
    intermedios = []

    for filename in os.listdir(CONFIG_DIR):
        if not filename.endswith(".txt"):
            continue

        config_path = os.path.join(CONFIG_DIR, filename)
        data_file, start, end, constraint = parse_config(config_path)

        if data_file.endswith("USA.txt") or data_file.endswith("W.txt"):
            print(f"⏭ Saltando {filename} (grafo: {data_file})")
            continue

        print(f"🚀 Ejecutando {filename}: {data_file} {start} → {end} con recurso {constraint}")

        try:
            output = ejecutar_instancia(data_file, start, end, constraint)
            algoritmo_data = parse_output(output)

            for alg in algoritmo_data:
                alg["Config"] = filename

            # Guardar resultados intermedios
            df_partial = pd.DataFrame(algoritmo_data)
            inter_path = os.path.join(RESULTS_DIR, f"{filename.replace('.txt', '')}.csv")
            df_partial.to_csv(inter_path, index=False)
            intermedios.append(inter_path)

            # Liberar memoria (opcional en este caso, pero útil si acumulas muchas instancias)
            del output, algoritmo_data, df_partial

        except Exception as e:
            print(f"❌ Error ejecutando {filename}: {e}")

    # Unir todos los resultados parciales
    print(f"📦 Uniendo {len(intermedios)} resultados parciales...")
    df_all = pd.concat([pd.read_csv(f) for f in intermedios], ignore_index=True)
    df_all.to_csv("comparativa_algoritmos.csv", index=False)

    if __name__ == "__main__":
         main()
