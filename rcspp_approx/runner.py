import os
import re
import subprocess
import pandas as pd

CONFIG_DIR = "/mnt/c/Users/snmca/gradle_projects/4208_ANALISIS_DE_ALGORITMOS/Algoritmo_approx_rcspp/instances"
CARGO_PROJECT_DIR = "/mnt/c/Users/snmca/gradle_projects/4208_ANALISIS_DE_ALGORITMOS/Algoritmo_approx_rcspp/rcspp_approx"
RESULTS_DIR = "/mnt/c/Users/snmca/gradle_projects/4208_ANALISIS_DE_ALGORITMOS/Algoritmo_approx_rcspp/rcspp_approx/resultados"
DEBUG_DIR = "/mnt/c/Users/snmca/gradle_projects/4208_ANALISIS_DE_ALGORITMOS/Algoritmo_approx_rcspp/rcspp_approx/debug_output"

os.makedirs(RESULTS_DIR, exist_ok=True)
os.makedirs(DEBUG_DIR, exist_ok=True)

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
    return result.stdout + result.stderr  # Captura también errores

def parse_output(output):
    bloques = output.split("Corriendo")[1:]
    if not bloques:
        print("⚠️ No se encontraron bloques de algoritmos en el output.")
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

            # Mostrar primeras líneas del output
            print("🧪 Primeras líneas del output:")
            print("\n".join(output.splitlines()[:10]))

            # Guardar output completo para depuración
            debug_path = os.path.join(DEBUG_DIR, f"{filename.replace('.txt', '')}.log")
            with open(debug_path, "w") as f:
                f.write(output)

            # Parsear resultado
            algoritmo_data = parse_output(output)

            if not algoritmo_data:
                print(f"⚠️ No se extrajo información útil de {filename}")
                continue

            for alg in algoritmo_data:
                alg["Config"] = filename

            # Guardar archivo intermedio
            df_partial = pd.DataFrame(algoritmo_data)
            inter_path = os.path.join(RESULTS_DIR, f"{filename.replace('.txt', '')}.csv")
            df_partial.to_csv(inter_path, index=False)
            intermedios.append(inter_path)

             # Suponiendo que algoritmo_data ya contiene los datos de esta ejecución
            for alg in algoritmo_data:
                alg["Config"] = filename
                alg["RecursoMáximo"] = int(constraint)
                alg["UsoRecursoRelativo"] = alg["Consumo"] / int(constraint) if alg["Consumo"] else None

            # Calcular calidad relativa comparada con el primer algoritmo (como óptimo base)
            costo_base = algoritmo_data[0]["Costo"]
            for alg in algoritmo_data:
                alg["Aprox_vs_base"] = alg["Costo"] / costo_base if alg["Costo"] and costo_base else None

            # Agregar a tabla parcial
            df_entry = pd.DataFrame(algoritmo_data)
            tabla_path = "tabla_parcial.csv"
            if os.path.exists(tabla_path):
                df_acum = pd.read_csv(tabla_path)
                df_acum = pd.concat([df_acum, df_entry], ignore_index=True)
            else:
                df_acum = df_entry

            df_acum.to_csv(tabla_path, index=False)
            print(f"📈 Tabla parcial actualizada ({len(df_acum)} filas)")

            # Liberar memoria
            del output, algoritmo_data, df_partial

        except Exception as e:
            print(f"❌ Error ejecutando {filename}: {e}")

    # Unir resultados intermedios
    print(f"📦 Uniendo {len(intermedios)} resultados parciales...")
    if intermedios:
        df_all = pd.concat([pd.read_csv(f) for f in intermedios], ignore_index=True)
        df_all.to_csv("comparativa_algoritmos.csv", index=False)
        print("✅ Archivo final guardado como comparativa_algoritmos.csv")
    else:
        print("⚠️ No se generaron archivos intermedios con datos válidos.")



if __name__ == "__main__":
    main()
