import * as mc from "@minecraft/server";
import * as ui from "@minecraft/server-ui";

mc.world.afterEvents.playerPlaceBlock.subscribe((data) => {
  const { player, block, dimension } = data;
  resolveBlock(block);
  const west = block.west();
  const east = block.east();
  const north = block.north();
  const south = block.south();
  if (!north?.isAir) resolveBlock(north!);
  if (!south?.isAir) resolveBlock(south!);
  if (!east?.isAir) resolveBlock(east!);
  if (!west?.isAir) resolveBlock(west!);
});

mc.world.afterEvents.playerBreakBlock.subscribe((data) => {
  const { player, block, dimension } = data;
  //   resolveBlock(block, true);
  const west = block.west();
  const east = block.east();
  const north = block.north();
  const south = block.south();
  if (!north?.isAir) resolveBlock(north!);
  if (!south?.isAir) resolveBlock(south!);
  if (!east?.isAir) resolveBlock(east!);
  if (!west?.isAir) resolveBlock(west!);
});

function resolveBlock(block: mc.Block, broken = false) {
  if (
    (block.typeId.includes("chipped:") &&
      block.typeId.includes("glass_pane")) ||
    broken
  ) {
    const west = block.west();
    const east = block.east();
    const north = block.north();
    const south = block.south();

    if (!west?.isAir) {
      //   console.warn("found west");
      block.setPermutation(
        block.permutation.withState("chipped:west_visible", true)
      );
    } else {
      block.setPermutation(
        block.permutation.withState("chipped:west_visible", false)
      );
    }

    if (!east?.isAir) {
      //   console.warn("found east");
      block.setPermutation(
        block.permutation.withState("chipped:east_visible", true)
      );
    } else {
      block.setPermutation(
        block.permutation.withState("chipped:east_visible", false)
      );
    }

    if (!north?.isAir) {
      //   console.warn("found north");
      block.setPermutation(
        block.permutation.withState("chipped:north_visible", true)
      );
    } else {
      block.setPermutation(
        block.permutation.withState("chipped:north_visible", false)
      );
    }

    if (!south?.isAir) {
      //   console.warn("found east");
      block.setPermutation(
        block.permutation.withState("chipped:south_visible", true)
      );
    } else {
      block.setPermutation(
        block.permutation.withState("chipped:south_visible", false)
      );
    }
  }
}

function* resolveGen(block: mc.Block) {
  resolveBlock(block);
  yield;
}

function fetchNearby4Chunks(location: mc.Vector3, dimension: mc.Dimension) {
  for (let x = -32; x <= 32; x++) {
    for (let y = -32; y <= 32; y++) {
      for (let z = -32; z <= 32; z++) {
        const block = dimension.getBlock({
          x: location.x + x,
          y: location.y + y,
          z: location.z + z,
        });
        if (!block) return;
        resolveBlock(block);
      }
    }
  }
}

mc.system.afterEvents.scriptEventReceive.subscribe((data) => {
  if (data.id == "glass_pane:fetch_nearby") {
    const location = data.sourceEntity!.location;
    const dimension = data.sourceEntity!.dimension;
    fetchNearby4Chunks(location, dimension);
  }
});
