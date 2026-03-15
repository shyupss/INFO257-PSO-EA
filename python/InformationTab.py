import pygame

class InformationTab():
    def __init__(self, position = (10, 10), padding = 10):
        self.position = position
        self.padding = padding
        self.font = pygame.font.SysFont("consolas", 12)

        self.bg_color = (0, 0, 0, 150)
        self.text_color = (255, 255, 255)

        self.border_radius = 10


    def render(self, screen: pygame.Surface, information: dict[str, int]):
        lineas = []

        for header, info in information.items():
            texto = f"{header}: {info}"
            surface = self.font.render(texto, True, self.text_color)
            lineas.append(surface)

        width = max(linea.get_width() for linea in lineas) + (self.padding * 2)
        height = sum(linea.get_height() for linea in lineas) + (self.padding * 2) + (len(lineas) - 1) * 4

        panel = pygame.Surface((width, height), pygame.SRCALPHA)

        pygame.draw.rect(
            panel,
            self.bg_color,
            panel.get_rect(),
            border_radius = self.border_radius
        )

        y = self.padding
        for linea in lineas:
            panel.blit(linea, (self.padding, y))
            y += linea.get_height() + 4

        screen.blit(panel, self.position)