<script lang="ts">
  import type { EncounterEntity } from '$lib/types'; // We'll define types later

  export let entities: EncounterEntity[] = [];
  export let currentTurnId: string = '';

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  function selectEntity(id: string) {
    dispatch('select', { id });
  }
</script>

<div class="sidebar">
  <h3>Initiative Order</h3>
  {#each entities as entity}
    <div 
      class="entity-row" 
      class:active={entity.instance_id === currentTurnId}
      on:click={() => selectEntity(entity.instance_id)}
    >
      <span>{entity.display_name}</span>
      <span>Initiative: {entity.initiative}</span>
      {#if entity.current_hp !== null}
        <span>HP: {entity.current_hp}</span>
      {/if}
    </div>
  {/each}
</div>

<style>
  .entity-row {
    padding: 8px;
    border-bottom: 1px solid #ccc;
    cursor: pointer;
  }
  .entity-row.active {
    background-color: #e0f7fa;
    font-weight: bold;
  }
</style>