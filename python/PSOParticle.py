import pygame
import random
import math
from Rastrigin import Rastrigin

class Particle:
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


    def move(self, screen, gbest, w = 0.7, c1 = 1.4, c2 = 1.4, max_vel = 4):
        # gbest = Mejor posición vista por cualquier partícula
        # w = Inercia
        # c1 = Factor de aprendizaje individual
        # c2 = Factor de aprendizaje según el éxito global 
        # max_vel = Velocidad máxima que puede tomar la partícula

        px, py = random.random(), random.random()

        # Actualizar la velocidad
        self.vx =  w * self.vx + c1 * px * (self.px - self.x) + c2 * px * (gbest[0] - self.x)
        self.vy =  w * self.vy + c1 * py * (self.py - self.y) + c2 * py * (gbest[1] - self.y)

        # self.vx = w * self.vx + px * (self.px - self.x) + px * (gbest[0] - self.x)
        # self.vy = w * self.vy + py * (self.py - self.y) + py * (gbest[1] - self.y)
        
        # Regular la velocidad de la partícula
        speed = math.sqrt(self.vx ** 2 + self.vy ** 2)
        if speed > max_vel:
            self.vx = self.vx / speed * max_vel
            self.vy = self.vy / speed * max_vel

        # Actualizar la posición de la partícula
        self.x += self.vx
        self.y += self.vy

        # Manejo de bordes
        width, height = screen.get_size()
        self.x = max(0, min(width - 1, self.x))
        self.y = max(0, min(height - 1, self.y))