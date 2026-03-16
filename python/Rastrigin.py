import pygame
import math
import numpy as np

class Rastrigin:
    def __init__(self, width, height, dominio_min = -3, dominio_max = 7):
        self.width = width
        self.height = height

        self.dominio_min = dominio_min
        self.dominio_max = dominio_max
        self.dominio_size = dominio_max - dominio_min

        self.imagen = pygame.Surface((width, height))


    @staticmethod
    def evaluate(x, y):
        return 20 + (x**2 - 10 * math.cos(2 * math.pi * x)) + (y**2 - 10 * math.cos(2 * math.pi * y))

    
    def generate(self):
        valores = []

        for py in range(self.height):
            fila = []
            for px in range(self.width):
                x = self.dominio_min + (px / self.width) * self.dominio_size
                y = self.dominio_min + (py / self.height) * self.dominio_size

                f = self.evaluate(x, y)
                fila.append(f)
            
            valores.append(fila)

        min_val = min(min(r) for r in valores)
        max_val = max(max(r) for r in valores)

        for py in range(self.height):
            for px in range(self.width):
                f = valores[py][px]
                t = (f - min_val) / (max_val - min_val)
                color = self.color_map(t)
                self.imagen.set_at((px, py), color)

        return self.imagen


    def color_map(self, t):
        r = int(255 * t)
        g = int(255 * (1 - abs(t - 0.5) * 2))
        b = int(255 * (1 - t))

        return (r, g, b)