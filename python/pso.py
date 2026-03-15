import pygame
import random

from Particle import Particle

VELOCITY = 3

class Simulacion:
    def __init__(self):
        pygame.init()

        self.width = 1000
        self.height = 700

        self.screen = pygame.display.set_mode((self.width, self.height))
        pygame.display.set_caption("PSO")
        self.bg = pygame.transform.scale(pygame.image.load("assets/heatmap.png").convert(), (self.width, self.height + 100))

        self.particles = [
            Particle() for _ in range(50)
        ]

        self.gbest = (0, 0)
        self.gbest_fitness = -999999

        self.clock = pygame.time.Clock()
        self.running = True

    # Main loop
    def run(self):
        while self.running:
            self.handle_events()
            self.update()
            self.render()
            self.clock.tick(60)  # 60 FPS

        pygame.quit()


    # Procesar los eventos del usuario
    def handle_events(self):
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False


    # Actualizar lógica
    def update(self):

        # Evaluar el fitness
        for particle in self.particles:
            fitness = particle.evaluate(self.bg)

            if fitness > self.gbest_fitness:
                self.gbest_fitness = fitness
                self.gbest = (particle.x, particle.y)

        # Mover las partículas
        for particle in self.particles:
            particle.move(self.screen, self.gbest)


    # Renderizar assets y objetos
    def render(self):
        self.screen.blit(self.bg, (0, 0))  # imagen de fondo

        for particle in self.particles:
            particle.draw(self.screen)

        pygame.display.flip()


if __name__ == "__main__":
    sim = Simulacion()
    sim.run()