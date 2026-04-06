import matplotlib.pyplot as plt
import numpy as np
from datetime import datetime

def graficar_convergencia(historiales_raw, titulo: str, xlabel: str, algorithm: str, params: dict = None):
    historiales = [h for h, _ in historiales_raw]
    best_steps = [b for _, b in historiales_raw]

    fig, ax = plt.subplots(figsize = (9, 5))

    for i, h in enumerate(historiales):
        ax.plot(h, color = "steelblue", alpha = 0.3, linewidth = 1, label = "_nolegend_")

    # Promedio
    min_len = min(len(h) for h in historiales)
    matriz = np.array([h[:min_len] for h in historiales])
    promedio = matriz.mean(axis = 0)
    mejor = matriz.max(axis = 0)

    ax.plot(promedio, color = "steelblue", linewidth = 2, label = "Promedio")
    ax.plot(mejor, color = "orange", linewidth = 2, linestyle = "--", label = "Mejor global")

    ax.set_title(titulo)
    ax.set_xlabel(xlabel)
    ax.set_ylabel("Fitness")
    ax.legend()
    ax.grid(True, alpha = 0.3)

    if params:
        promedio_best = sum(best_steps) / len(best_steps)
        label_best = "Mejor iteración (prom.)" if xlabel == "Iteración" else "Mejor generación (prom.)"
        params[label_best] = f"{promedio_best:.1f}"
        
        texto = "\n".join(f"{k}: {v}" for k, v in params.items())
        ax.text(
            1.02, 1,
            texto,
            transform = ax.transAxes,
            fontsize = 9,
            verticalalignment = "top",
            bbox = dict(boxstyle = "round", facecolor = "whitesmoke", alpha = 0.8)
        )
        fig.subplots_adjust(right = 0.75)


    save_path = f"graficos/{algorithm}_{datetime.now().strftime('%d-%m-%Y_%H-%M-%S')}.png"

    plt.tight_layout()
    plt.savefig(save_path)

    return save_path