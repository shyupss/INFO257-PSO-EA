import pygame
import random
import math

class Particle:
    def __init__(self, radius = 8, color = (170, 0, 255)):
        self.color = color
        self.radius = radius

        self.image = pygame.image.load("assets/particle.png").convert_alpha()

        # Posición
        self.x = random.uniform(0, 1000)
        self.y = random.uniform(0, 700)

        # Velocidad
        self.vx = random.uniform(-1, 1)
        self.vy = random.uniform(-1, 1)

        # Posición del personal best
        self.px = self.x
        self.py = self.y

        # Personal best
        self.pbest = -999999


    def draw(self, screen):
        screen.blit(self.image, dest = (int(self.x), int(self.y)))
        # pygame.draw.circle(
        #     screen,
        #     self.color,
        #     (int(self.x), int(self.y)),
        #     self.radius,
        #     width = 2
        # )


    def evaluate(self, background: pygame.Surface):
        # Obtener los colores de la imagen (mapa de calor)
        r, g, b, _ = background.get_at((int(self.x), int(self.y)))

        # b - r -> Buscar las zonas más azules
        fitness = b - r - g

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
        self.vx = w * self.vx + c1 * px * (self.px - self.x) + c2 * px * (gbest[0] - self.x)
        self.vy = w * self.vy + c1 * py * (self.py - self.y) + c2 * py * (gbest[1] - self.y)
        
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