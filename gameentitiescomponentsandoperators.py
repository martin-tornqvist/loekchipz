### gameentitiescomponentsandoperators.py
## sodr 20171010204238
# emacs compile   python gameentitiescomponentsandoperators.py


import os
import sys
import math
import random


class EntityComponent(object):
    def __init__(self):
        self.name = None
        self.prepare = None
        self.operate = None
        self.adjust = None


class Entity(object):
    id_counter = -1
    def __init__(self):
        # components
        Entity.id_counter += 1
        self.id = Entity.id_counter
        self.name = None
        self.components = []


class Game(object):
    def __init__(self):
        self.entities_live = []
        self.entities_dead = []


def text_from_physics_component(p):
    t = '{PhysicsComponent'
    t += ' {.minv ' + str(p.minv) + '}'
    t += ' {.fx ' + str(p.fx) + '}'
    t += ' {.fy ' + str(p.fy) + '}'
    t += ' {.ax ' + str(p.ax) + '}'
    t += ' {.ay ' + str(p.ay) + '}'
    t += ' {.vx ' + str(p.vx) + '}'
    t += ' {.vy ' + str(p.vy) + '}'
    t += ' {.px ' + str(p.px) + '}'
    t += ' {.py ' + str(p.py) + '}'
    t += '}'
    return t


class PhysicsComponent(EntityComponent):
    name = "physics"
    def __init__(self):
        EntityComponent.__init__(self)
        self.name = PhysicsComponent.name

        # mass inverse
        self.minv = 0

        # force
        self.fx = 0
        self.fy = 0

        # acceleration
        self.ax = 0
        self.ay = 0

        # velocity
        self.vx = 0
        self.vy = 0

        # position
        self.px = 0
        self.py = 0


    def __str__(self):
        return text_from_physics_component(self)


def text_from_graphics_component(g):
    t = '{GraphicsComponent'
    t += ' {.tex ' + str(g.tex) + '}'
    t += ' {.tint ' + str(g.tint) + '}'
    t += '}'
    return t


class GraphicsComponent(EntityComponent):
    name = "graphics"
    def __init__(self):
        EntityComponent.__init__(self)
        self.name = GraphicsComponent.name

        # texture
        self.tex = -1

        self.tint = (1.0, 1.0, 1.0)

    def __str__(self):
        return text_from_graphics_component(self)


def entity_component_insert(entity, component):
    if not entity:
        return
    for c in entity.components:
        if c.name == component.name:
            # already holds this type of component
            return
    entity.components.append(component)


def entity_component_remove(entity, coname):
    if not entity:
        return
    for c in entity.components:
        if c.name == coname:
            entity.components.remove(c)
            return


def components_from_entity(entity, coname):
    if not entity:
        return None
    cos = []
    for c in entity.components:
        if c.name == coname:
            cos.append(c)
    return cos


def physics_component_prepare_1(game, entity, physicscomponent):
    print('           prepare_1')
    t = game.time_game_frame_delta
    physicscomponent.fx = (2 * random.random()) - 1
    physicscomponent.fy = 100


def physics_component_operate_1(game, entity, physicscomponent):
    print('           operate_1')
    assert game
    assert physicscomponent
    # assert entity
    t = game.time_game_frame_delta
    p = physicscomponent
    p.ax += p.fx * p.minv
    p.ay += p.fy * p.minv
    p.vx += p.ax * t
    p.vy += p.ay * t
    p.px += p.vx
    p.py += p.vy


def physics_component_adjust_1(game, entity, physicscomponent):
    print('           adjust_1')
    floor = 4.6
    if physicscomponent.py < floor:
        physicscomponent.py = floor


def frame_prepare(game, entity, component):
    if component.prepare:
        component.prepare(game, entity, component)


def frame_operate(game, entity, component):
    if component.operate:
        component.operate(game, entity, component)


def frame_adjustments(game, entity, component):
    if component.adjust:
        component.adjust(game, entity, component)


def frame_render_entity(game, entity):
    p = components_from_entity(entity, PhysicsComponent.name)
    if not p:
        return
    assert 1 == len(p)
    p = p[0]
    g = components_from_entity(entity, GraphicsComponent.name)
    if not g:
        return
    assert 1 == len(g)
    g = g[0]
    print('graphics:             opengl bind texture "' + str(g.tex) + '"')
    print('graphics:             opengl accumulate sprite to {px ' + str(p.px) + '} {py ' + str(p.py) + '}')


def game_load(game):
    print('game_load start')
    entities_to_create = 3
    for i in range(entities_to_create):
        e = Entity()
        e.name = 'rocket'
        pc = PhysicsComponent()
        pc.minv = 1.0
        pc.prepare = physics_component_prepare_1
        pc.operate = physics_component_operate_1
        pc.adjust  = physics_component_adjust_1
        for i in range(int(1e5)):
            entity_component_insert(e, pc)
            entity_component_remove(e, PhysicsComponent.name)
        entity_component_insert(e, pc)
        pcs = components_from_entity(e, PhysicsComponent.name)
        t = '{pcs'
        for pc in pcs:
            t += ' ' + str(pc)
        t += '}'
        print(t)

        gc = GraphicsComponent()
        entity_component_insert(e, gc)
        gcs = components_from_entity(e, GraphicsComponent.name)
        t = '{gcs'
        for gc in gcs:
            t += ' ' + str(gc)
        t += '}'
        print(t)

        game.entities_live.append(e)
    print('game_load end')


def game_loop_iteration(game):
    print('------------------------------------------------------------')
    print('game_loop_iteration')
    g = game
    g.time_game_frame_delta = 1.0 / 30.0
    for f in [
            frame_prepare,
            frame_operate,
            frame_adjustments,
    ]:
        for e in g.entities_live:
            for c in e.components:
                f(g, e, c)
    print('graphics: prepare render target')
    ents = g.entities_dead + g.entities_live
    for e in ents:
        frame_render_entity(g, e)
    print('graphics: swap buffer')
    print('------------------------------------------------------------')


def game_unload(game):
    print('game_unload')


def main():
    g = Game()
    game_load(g)
    for i in range(4):
        game_loop_iteration(g)
    game_unload(g)


if __name__ == '__main__':
    main()

