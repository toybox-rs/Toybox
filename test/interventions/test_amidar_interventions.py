from unittest import TestCase
from ctoybox import Toybox, Input
import toybox.interventions.amidar as amidar
from toybox.interventions.amidar import AmidarIntervention, Amidar
from toybox.interventions.base import MutationError, InterventionNoneError

class AmidarInterventionTests(TestCase):

  def setUp(self):
    self.tb = Toybox('amidar')

    fire = Input()
    fire.button1 = True
    noop = Input()
    self.tb.apply_action(fire)

  def test_allowable_interventions(self):
    with AmidarIntervention(self.tb) as intervention:
      with self.assertRaises(InterventionNoneError):
        intervention.game.board.intervention = None

      with self.assertRaises(MutationError):
        intervention.game.board.intervention = intervention

      with self.assertRaises(MutationError):
        intervention.game.board._in_init = True
          # assert False, 'We should not be able to manulaly set the _in_init flag'

      self.assertIn('intervention', intervention.game.board.immutable_fields)
      self.assertNotIn('_in_init', intervention.game.board.immutable_fields)

  def test_dirty_state(self):
    with AmidarIntervention(self.tb) as intervention:
      intervention.game.lives = 1
      self.assertTrue(intervention.dirty_state)

  def test_clean_state(self):
    with AmidarIntervention(self.tb) as intervention:
      self.assertEqual(intervention.get_tile_by_pos(0, 0).tag, amidar.Tile.ChaseMarker)
      self.assertFalse(intervention.dirty_state)

  def test_random_track_position(self):
    with AmidarIntervention(self.tb) as intervention:
      pos1 = intervention.get_random_track_position()
      pos2 = intervention.get_random_track_position()
      self.assertNotEqual(pos1, pos2)

  def test_player_random_start(self):
    with AmidarIntervention(self.tb) as intervention:
      player_pos1 = intervention.game.player.position
      player_pos2 = intervention.game.player.position
      self.assertEqual(player_pos1, player_pos2)
      intervention.set_player_random_start()
      player_pos2 = intervention.game.player.position
      self.assertNotEqual(player_pos1, player_pos2)
      self.assertTrue(intervention.dirty_state)

  def test_painting(self):
    with AmidarIntervention(self.tb) as intervention:
      tile = intervention.get_tile_by_pos(tx=0, ty=0)
      tile_marker_before = tile.tag
      intervention.set_tile_tag(tile, amidar.Tile.Painted)
      tile_market_after = tile.tag
      self.assertNotEqual(tile_marker_before, tile_market_after)
      self.assertTrue(intervention.dirty_state)

  def test_unpainting(self):
    with AmidarIntervention(self.tb) as intervention:
      tile = intervention.get_tile_by_pos(0, 0)
      tile_marker_before = tile.tag
      intervention.set_tile_tag(tile, amidar.Tile.Painted)
      intervention.set_tile_tag(tile, amidar.Tile.ChaseMarker)
      tile_marker_after = tile.tag
      self.assertEqual(tile_marker_before, tile_marker_after)
      self.assertTrue(intervention.dirty_state)


  def test_get_number_enemies(self):
    # get number of enemies
    with AmidarIntervention(self.tb) as intervention: 
      self.assertEqual(len(intervention.game.enemies), 5)
      self.assertFalse(intervention.dirty_state)

  def test_remove_enemy(self):
    with AmidarIntervention(self.tb) as intervention:
      enemies = intervention.game.enemies
      enemies.remove(enemies[4])
      self.assertEqual(len(enemies), len(intervention.game.enemies))
      self.assertTrue(intervention.dirty_state)
          # check number of enemies
    with AmidarIntervention(self.tb) as intervention: 
      self.assertEqual(len(intervention.game.enemies), 4)
      self.assertFalse(intervention.dirty_state)

  def test_add_enemy(self):
    # add enemy with 'EnemyLookupAI' protocol
    with AmidarIntervention(self.tb) as intervention: 
      enemies = intervention.game.enemies
      # copy the second enemy
      enemy = amidar.Enemy.decode(intervention, enemies[1].encode(), amidar.Enemy)
      next = max([e.ai.next for e in enemies]) + 1
      # Not sure what default route index refers to, so I am picking an arbitrary number
      default_route_index = 10
      intervention.set_enemy_protocol(enemy, amidar.MovementAI.EnemyLookupAI, 
        next=next, default_route_index=default_route_index) 
      enemies.append(enemy)
      self.assertTrue(intervention.dirty_state)

    with AmidarIntervention(self.tb) as intervention: 
      self.assertEqual(len(intervention.game.enemies), 6)
      self.assertFalse(intervention.dirty_state)

  def test_change_enemy_protocol(self):
    import random
    # change to 'EnemyPerimeterAI' protocol
    with AmidarIntervention(self.tb) as intervention:
      enemy = intervention.game.enemies[-1]
      intervention.set_enemy_protocol(enemy, amidar.MovementAI.EnemyPerimeterAI,
        start=amidar.TilePoint(intervention, tx=0, ty=0))
      self.assertTrue(intervention.dirty_state)
    with AmidarIntervention(self.tb) as intervention: 
      self.assertEqual(intervention.game.enemies[-1].ai.protocol, amidar.MovementAI.EnemyPerimeterAI)
      self.assertFalse(intervention.dirty_state)

    
    # change to 'EnemyAmidarMvmt' protocol
    with AmidarIntervention(self.tb) as intervention: 
      enemy = intervention.game.enemies[-1]
      intervention.set_enemy_protocol(enemy, 'EnemyAmidarMvmt',
        vert=amidar.Direction(intervention, random.choice(amidar.Direction.directions)),
        horiz=amidar.Direction(intervention, random.choice(amidar.Direction.directions)),
        start_vert=amidar.Direction(intervention, random.choice(amidar.Direction.directions)),
        start_horiz=amidar.Direction(intervention, random.choice(amidar.Direction.directions)),
        start=amidar.TilePoint.decode(intervention, enemy.ai.start, amidar.TilePoint)
        )
      self.assertTrue(intervention.dirty_state)

    with AmidarIntervention(self.tb) as intervention: 
      self.assertEqual(intervention.game.enemies[-1].ai.protocol, amidar.MovementAI.EnemyAmidarMvmt)
      self.assertFalse(intervention.dirty_state)

    # change to 'EnemyTargetPlayer' protocol
    with AmidarIntervention(self.tb) as intervention: 
      enemy = intervention.game.enemies[-1]
      intervention.set_enemy_protocol(enemy, 'EnemyTargetPlayer',
        start=amidar.TilePoint.decode(intervention, enemy.ai.start, amidar.TilePoint),
        vision_distance=10,
        player_seen=None,
        start_dir=amidar.Direction(intervention, random.choice(amidar.Direction.directions)),
        dir=amidar.Direction(intervention, random.choice(amidar.Direction.directions))
      )
      self.assertTrue(intervention.dirty_state)
    with AmidarIntervention(self.tb) as intervention: 
      self.assertEqual(intervention.game.enemies[-1].ai.protocol, amidar.MovementAI.EnemyTargetPlayer)
      self.assertFalse(intervention.dirty_state)

    # change to 'EnemyRandomAI' protocol
    with AmidarIntervention(self.tb) as intervention: 
      enemy = intervention.game.enemies[-1]
      intervention.set_enemy_protocol(enemy, 'EnemyRandomMvmt',
        start=amidar.TilePoint.decode(intervention, enemy.ai.start, amidar.TilePoint),
        start_dir=amidar.Direction(intervention, random.choice(amidar.Direction.directions)),
        dir=amidar.Direction(intervention, random.choice(amidar.Direction.directions)),
      )
      self.assertTrue(intervention.dirty_state)
    with AmidarIntervention(self.tb) as intervention: 
      self.assertEqual(intervention.game.enemies[-1].ai.protocol, amidar.MovementAI.EnemyRandomMvmt)
      self.assertFalse(intervention.dirty_state)

  def test_num_jumps(self):
    # check number of jumps
    with AmidarIntervention(self.tb) as intervention: 
      self.assertEqual(intervention.game.jumps, 3)
      intervention.game.jumps = 5
      self.assertTrue(intervention.dirty_state)
    with AmidarIntervention(self.tb) as intervention:            
      self.assertEqual(intervention.game.jumps, 5)
      self.assertFalse(intervention.dirty_state)

  def test_jump_mode(self):
    # check jump mode
    with AmidarIntervention(self.tb) as intervention:
      intervention.set_mode('jump')
      self.assertTrue(intervention.dirty_state)
    with AmidarIntervention(self.tb) as intervention:
      self.assertGreater(intervention.game.jump_timer, 0)
      self.assertFalse(intervention.dirty_state)

  def test_random_starts(self):
      # check random starts
    with AmidarIntervention(self.tb) as intervention:
      initial_start = intervention.game.player.position
      self.assertFalse(intervention.dirty_state)
    with AmidarIntervention(self.tb) as intervention:
      intervention.set_player_random_start()
      self.assertTrue(intervention.dirty_state)
      wp = intervention.game.player.position
      self.assertTrue(wp.x != initial_start.x or wp.y != initial_start.y)

  def test_immutable_fields(self):
    with AmidarIntervention(self.tb) as intervention:
      with self.assertRaises(amidar.InterventionNoneError):
        intervention.game.player.intervention = None
      with self.assertRaises(amidar.MutationError):
        intervention.game.player._in_init = True