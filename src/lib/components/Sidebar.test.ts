import { render, screen, fireEvent } from 'svelte-testing-library';
import Sidebar from './Sidebar.svelte';

import type { EncounterEntity } from '$lib/types';

describe('Sidebar', () => {
  const mockEntities: EncounterEntity[] = [
    {
      instance_id: 'e1',
      display_name: 'Goblin #1',
      entity_type: 'monster',
      initiative: 12,
      current_hp: 7,
      max_hp: 7,
      monster_id: 'goblin',
      conditions: [],
      is_active: true
    }
  ];

  it('renders entity names', () => {
    render(Sidebar, { props: { entities: mockEntities, currentTurnId: '' } });
    expect(screen.getByText('Goblin #1')).toBeInTheDocument();
  });

  it('highlights current turn entity', () => {
    render(Sidebar, { props: { entities: mockEntities, currentTurnId: 'e1' } });
    const row = screen.getByText('Goblin #1').closest('div');
    expect(row?.classList.contains('active')).toBe(true);
  });
});