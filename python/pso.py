import pygame
import random

from Particle import Particle

VELOCITY = 3
W = 200


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