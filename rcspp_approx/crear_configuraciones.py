#!/usr/bin/env python3
import os
import argparse
import heapq
from typing import List, Tuple, Callable, Optional

# --- Dijkstra genérico -----------------------------------
def normal_dijkstra(
    graph: List[List[Tuple[int, int, int]]],
    s: int,
    e: int,
    cost_func: Callable[[Tuple[int, int, int]], int]
) -> Tuple[int,int]:
    """
    Devuelve (min_cost, consumo_en_camino_min_cost)
    graph[u] = list of (v, cost, cons)
    """
    # heap entries: (acum_cost, nodo, acum_cons)
    heap: List[Tuple[int,int,int]] = [(0, s, 0)]
    best_cost = [float('inf')] * len(graph)
    best_cost[s] = 0

    while heap:
        dist_u, u, cons_u = heapq.heappop(heap)
        if dist_u > best_cost[u]:
            continue
        if u == e:
            return dist_u, cons_u
        for v, cost, cons in graph[u]:
            c = cost_func((v, cost, cons))
            nd = dist_u + c
            if nd < best_cost[v]:
                best_cost[v] = nd
                heapq.heappush(heap, (nd, v, cons_u + cons))
    # si no hay camino, devolvemos (∞, ∞)
    return float('inf'), float('inf')


# --- Carga grafo y cuenta arcos/nodos ----------------------
def load_graph(path: str) -> Tuple[List[List[Tuple[int,int,int]]], int, int]:
    graph: List[List[Tuple[int,int,int]]] = []
    arcs = 0
    max_node = 0
    with open(path, 'r') as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            u,v,cost,cons = map(int, line.split())
            arcs += 1
            max_node = max(max_node, u, v)
            while len(graph) <= max_node:
                graph.append([])
            graph[u].append((v, cost, cons))
    return graph, arcs, max_node+1


# --- Main ---------------------------------------------------
def main():
    parser = argparse.ArgumentParser(
        description="Genera tabla y configs con tightness para cada Region/EndNode"
    )
    parser.add_argument(
        "lista", help="configuraciones.txt con: REGION  StartNode  EndNode"
    )
    parser.add_argument(
        "--dir", default=".", help="directorio donde están los USA-road-<REGION>.txt"
    )
    args = parser.parse_args()

    p_values = [0.1, 0.2, 0.4, 0.6, 0.8]
    resultados = []

    with open(args.lista) as f:
        for line in f:
            region, s_str, e_str = line.split()
            s, e = int(s_str), int(e_str)

            # 1) Cargo el grafo
            fname = os.path.join(args.dir, f"USA-road-{region}.txt")
            graph, num_arcs, num_nodes = load_graph(fname)

            # 2) Dijkstra minimizando COSTE => me devuelve (best_cost, consumo_en_ese_camino)
            best_cost, cons_on_cost_path = normal_dijkstra(
                graph, s, e, cost_func=lambda edge: edge[1]
            )
            # 3) Dijkstra minimizando CONSUMO => (cost_on_cons_path, best_cons)
            cost_on_cons_path, best_cons = normal_dijkstra(
                graph, s, e, cost_func=lambda edge: edge[2]
            )

            # 4) Calculo T(p) para cada tightness
            t_list = [
                best_cons + p * (cons_on_cost_path - best_cons)
                for p in p_values
            ]
            resultados.append((region, s, e, num_arcs, num_nodes, t_list))

            # 5) Genero un config_<REGION><EndNode><p>.txt por cada p
            for p, t in zip(p_values, t_list):
                pname = str(p).replace('.', '')
                cfg = f"config_{region}{e}{pname}.txt"
                with open(cfg, 'w') as cf:
                    cf.write(f"DataFile:USA-road-{region}.txt\n")
                    cf.write(f"NumberOfArcs:{num_arcs}\n")
                    cf.write(f"NumberOfNodes:{num_nodes}\n")
                    cf.write(f"TimeConstraint:{int(t)}\n")
                    cf.write(f"StartNode:{s}\n")
                    cf.write(f"EndNode:{e}\n")

    # 6) Escribo la tabla completa
    with open("tabla_completa.txt", "w") as tf:
        header = ["Region","StartNode","EndNode","NumArcs","NumNodes"] + [str(p) for p in p_values]
        tf.write("\t".join(header) + "\n")
        for region, s, e, arcs, nodes, t_list in resultados:
            row = [region, str(s), str(e), str(arcs), str(nodes)] + [str(int(t)) for t in t_list]
            tf.write("\t".join(row) + "\n")

    print("¡Generadas todas las configs y tabla_completa.txt!")


if __name__ == "__main__":
    main()