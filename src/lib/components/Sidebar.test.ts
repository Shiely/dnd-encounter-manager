import { render, screen, fireEvent } from '@testing-library/svelte';
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
    },
    {
      instance_id: 'e2',
      display_name: 'Player 1',
      entity_type: 'player',
      initiative: 18,
      current_hp: null,
      max_hp: null,
      monster_id: null,
      conditions: [],
      is_active: true
    }
  ];

  it('renders entity names', () => {
    render(Sidebar, { props: { entities: mockEntities, currentTurnId: '' } });
    expect(screen.getByText('Goblin #1')).toBeInTheDocument();
    expect(screen.getByText('Player 1')).toBeInTheDocument();
  });

  it('highlights current turn entity', () => {
    render(Sidebar, { props: { entities: mockEntities, currentTurnId: 'e1' } });
    const activeRow = screen.getByText('Goblin #1').closest('.entity-row');
    expect(activeRow).toHaveClass('active');
  });

  it('dispatches select event when clicked', async () => {
    const { component } = render(Sidebar, { props: { entities: mockEntities, currentTurnId: '' } });
    
    const selectHandler = vi.fn();
    component.$on('select', selectHandler);

    await fireEvent.click(screen.getByText('Goblin #1'));
    expect(selectHandler).toHaveBeenCalled();
  });
});