def amidar_num_tiles_unpainted(self):
        assert(self.game_name == 'amidar')
        return self.query_json('num_tiles_unpainted')
    
def amidar_player_tile(self):
    assert(self.game_name == 'amidar')
    return self.query_json('player_tile')

def amidar_num_enemies(self):
    assert(self.game_name == 'amidar')
    return self.query_json('num_enemies')

def amidar_jumps_remaining(self):
    assert(self.game_name == 'amidar')
    return self.query_json('jumps_remaining')

def amidar_regular_mode(self):
    assert(self.game_name == 'amidar')
    return self.query_json('regular_mode')

def amidar_jump_mode(self):
    assert(self.game_name == 'amidar')
    return self.query_json('jump_mode')

def amidar_chase_mode(self):
    assert(self.game_name == 'amidar')
    return self.query_json('chase_mode')

def amidar_enemy_tiles(self):
    assert(self.game_name == 'amidar')
    return self.query_json('enemy_tiles')

def amidar_enemy_caught(self, eid):
    assert(self.game_name == 'amidar')
    return self.query_json('enemy_caught', eid)

def amidar_any_enemy_caught(self, eid):
    assert(self.game_name == 'amidar')
    num_enemies = self.amidar_num_enemies()
    return any(self.amidar_enemy_caught(eid) for eid in range(num_enemies))