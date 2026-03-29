import pygame
import random
import numpy as np
from Rastrigin import Rastrigin

class Particle:
    '''
    Partícula para la simulación de el algoritmo evolutivo "Algoritmo Genético"
    '''

    def __init__(self, screen: pygame.Surface, radius = 4, color = (255, 255, 255)):
        self.color = color
        self.radius = radius

        self.image = pygame.image.load("assets/particle.png").convert_alpha()
        w, h = screen.get_size()
        self.width = w
        self.height = h

        # Posición
        self.x = random.uniform(0, w)
        self.y = random.uniform(0, h)

        # Velocidad
        self.vx = random.uniform(-1, 1)
        self.vy = random.uniform(-1, 1)

        # Posición del personal best
        self.px = self.x
        self.py = self.y

        # Personal best
        self.pbest = -999999


    def translate_coords_to_dom(self):
        return (
            -3 + (self.x / self.width) * 10,
            -3 + (self.y / self.height) * 10
        )

    def draw(self, screen):
        # screen.blit(self.image, dest = (int(self.x), int(self.y)))
        pygame.draw.circle(
            screen,
            self.color,
            (int(self.x), int(self.y)),
            self.radius
        )


    def evaluate(self):
        x, y = self.translate_coords_to_dom()
        fitness = -Rastrigin.evaluate(x, y)

        if fitness > self.pbest:
            self.pbest = fitness
            self.px = self.x
            self.py = self.y

        return fitness


    def move(self, screen, gbest):
        ...