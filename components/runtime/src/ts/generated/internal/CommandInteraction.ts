import type { CommandInteractionDataMap } from "./CommandInteractionDataMaps";
import type { CommandInteractionOption } from "./CommandInteractionOption";
import type { CommandType } from "./CommandType";
import type { IMember } from "./Member";

export interface CommandInteraction {
  channelId: string;
  id: string;
  member: IMember;
  token: string;
  name: string;
  parentName: string | null;
  parentParentName: string | null;
  options: Array<CommandInteractionOption>;
  dataMap: CommandInteractionDataMap;
  kind: CommandType;
  targetId: string | null;
}
