import pygame
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
        return 20 + (x**2 - 10 * np.cos(2 * np.pi * x)) + (y**2 - 10 * np.cos(2 * np.pi * y))

    
    def generate(self):
        x = np.linspace(self.dominio_min, self.dominio_max, self.width)
        y = np.linspace(self.dominio_min, self.dominio_max, self.height)

        X, Y = np.meshgrid(x, y)

        Z = self.evaluate(X, Y)
        Z = (Z - Z.min()) / (Z.max() - Z.min())

        image = np.dstack(self.color_map(Z))
        self.image = pygame.surfarray.make_surface(image.swapaxes(0, 1))

        return self.image


    def color_map(self, Z):
        R = (255 * Z).astype(np.uint8)
        G = (255 * (1 - np.abs(Z - 0.5) * 2)).astype(np.uint8)
        B = (255 * (1 - Z)).astype(np.uint8)

        return (R, G, B)