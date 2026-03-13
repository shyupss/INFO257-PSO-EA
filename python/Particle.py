import pygame
import random


class Particle:
    def __init__(self, x, y, radius = 8, color = (170, 0, 255)):
        self.pos = [x, y]
        self.vel = [random.randint(2, 5), random.randint(2, 5)]
        self.radius = radius
        self.color = color
        self.pbest = [0, 0]

    def draw(self, screen):
        pygame.draw.circle(
            screen,
            self.color,
            self.pos,
            self.radius,
            width = 2
        )

    def update_pos(self, width, height):
        # Actualizar posición según velocidad
        self.pos = [self.pos[0] + self.vel[0], self.pos[1] + self.vel[1]]

        # Rebotes con las paredes
        if self.pos[0] - self.radius < 0:
            self.pos[0] = self.radius
            self.vel[0] *= -1

        elif self.pos[0] + self.radius > width:
            self.pos[0] = width - self.radius
            self.vel[0] *= -1

        if self.pos[1] - self.radius < 0:
            self.pos[1] = self.radius
            self.vel[1] *= -1

        elif self.pos[1] + self.radius > height:
            self.pos[1] = height - self.radius
            self.vel[1] *= -1