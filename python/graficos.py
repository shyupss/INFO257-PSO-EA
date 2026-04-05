import matplotlib.pyplot as plt
import numpy as np
from datetime import datetime

def graficar_convergencia(historiales: list[list[float]], titulo: str, xlabel: str, algorithm: str):
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

    save_path = f"graficos/{algorithm}_{datetime.now().strftime('%d-%m-%Y_%H-%M-%S')}.png"

    plt.tight_layout()
    plt.savefig(save_path)

    return save_path