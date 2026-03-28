import pygame

from Particle import Particle
from InformationTab import InformationTab
from Rastrigin import Rastrigin

class Simulacion:
    def __init__(self):
        pygame.init()

        self.width = 700
        self.height = 700

        self.screen = pygame.display.set_mode((self.width, self.height))
        pygame.display.set_caption("PSO")
        #self.bg = pygame.image.load("assets/circulos.png").convert()
        rastrigin_map = Rastrigin(self.width, self.height)
        self.bg = rastrigin_map.generate()
        self.gbest_img = pygame.image.load("assets/global_best.png").convert_alpha()

        # Parámetros de las partículas
        self.learning_c1 = 3
        self.learning_c2 = 10
        self.inercia = 100
        self.max_velocity = 4

        self.gbest = (0, 0)
        self.gbest_fitness = -999999

        # Generación de partículas
        self.particles = [
            Particle(self.screen) for _ in range(200)
        ]

        # Panel de información
        self.information = InformationTab()

        self.clock = pygame.time.Clock()
        self.running = True
        self.iteracion = 0

    # Main loop
    def run(self):
        while self.running:
            self.handle_events()
            self.update()
            self.render()
            self.clock.tick(24)

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
            fitness = particle.evaluate()

            if fitness > self.gbest_fitness:
                self.gbest_fitness = fitness
                self.gbest = (particle.x, particle.y)

        # Mover las partículas
        for particle in self.particles:
            particle.move(self.screen, self.gbest,
                c1 = self.learning_c1,
                c2 = self.learning_c2,
                w = self.inercia,
                max_vel = self.max_velocity
            )

        self.iteracion += 1


    # Renderizar assets y objetos
    def render(self):
        self.screen.blit(self.bg, (0, 0))  # imagen de fondo

        # Dibujar partículas
        for particle in self.particles:
            particle.draw(self.screen)

        # Dibujar el global best
        self.screen.blit(self.gbest_img, dest = (int(self.gbest[0]), int(self.gbest[1])))

        # Dibujar información
        self.information.render(self.screen, 
            {
                "Iteración": self.iteracion,
                "Cantidad de partículas": len(self.particles),
                "Mejor posición global": f"({self.gbest[0]:0.2f}, {self.gbest[1]:0.2f})",
                "Fitness global": self.gbest_fitness,
                "Learning factor C1": self.learning_c1,
                "Learning factor C2": self.learning_c2,
                "Inercia W": self.inercia,
                "Velocidad máxima": self.max_velocity
            }
        )

        pygame.display.flip()


if __name__ == "__main__":
    sim = Simulacion()
    sim.run()