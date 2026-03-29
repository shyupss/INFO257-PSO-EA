import pygame

from InformationTab import InformationTab
from Rastrigin import Rastrigin

class Simulacion:
    '''
    Simulación del algoritmo evolutivo "Algoritmo genético"
    '''

    def __init__(self):
        pygame.init()

        self.width = 700
        self.height = 700

        self.screen = pygame.display.set_mode((self.width, self.height))
        pygame.display.set_caption("Algoritmo Genético")
        rastrigin_map = Rastrigin(self.width, self.height)
        self.bg = rastrigin_map.generate()
        self.gbest_img = pygame.image.load("assets/global_best.png").convert_alpha()

        # Parámetros de las partículas
        self.gbest = (0, 0)
        self.gbest_fitness = -999999

        # Generación de partículas
        #

        # Panel de información
        self.information = InformationTab()

        self.clock = pygame.time.Clock()
        self.running = True
        self.generacion = 0

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
            if event.type == pygame.KEYDOWN:
                # Avanzar generación
                ...
            if event.type == pygame.QUIT:
                self.running = False


    # Actualizar lógica
    def update(self):
        ...


    # Renderizar assets y objetos
    def render(self):
        # Dibujar fondo
        self.screen.blit(self.bg, (0, 0))

        # Dibujar partículas
        #

        # Dibujar el global best
        self.screen.blit(self.gbest_img, dest = (int(self.gbest[0]), int(self.gbest[1])))

        # Dibujar información
        self.information.render(self.screen, 
            {
                "Generación": self.generacion,
                "Cantidad de partículas": None,
                "Mejor posición global": f"({self.gbest[0]:0.2f}, {self.gbest[1]:0.2f})",
                "Fitness global": self.gbest_fitness,
            }
        )

        pygame.display.flip()


if __name__ == "__main__":
    sim = Simulacion()
    sim.run()