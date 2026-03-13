import pygame
import random

VELOCITY = 3
W = 200

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


class Simulacion:
    def __init__(self):
        pygame.init()

        self.width = 1000
        self.height = 700

        self.screen = pygame.display.set_mode((self.width, self.height))
        pygame.display.set_caption("PSO")
        self.bg = pygame.transform.scale(pygame.image.load("assets/heatmap.png").convert(), (self.width, self.height + 100))

        self.particles = [
            Particle(random.randint(0, self.width), random.randint(0, self.height)) for _ in range(50)
        ]

        self.gbest = [0, 0]

        self.clock = pygame.time.Clock()
        self.running = True

    def run(self):
        while self.running:
            self.handle_events()
            self.update()
            self.render()
            self.clock.tick(60)  # 60 FPS

        pygame.quit()

    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False

    def update(self):
        for particle in self.particles:
            particle.update_pos(self.width, self.height)

    def render(self):
        self.screen.blit(self.bg, (0, 0))  # color de fondo

        for particle in self.particles:
            particle.draw(self.screen)

        pygame.display.flip()


if __name__ == "__main__":
    sim = Simulacion()
    sim.run()