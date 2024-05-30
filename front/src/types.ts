// type WithId = {
//   readonly id: number;
// };
//
// type Proto<T extends WithId> = Omit<T, 'id'>;
//
// function addId<T extends WithId>(id: number, value: Proto<T>): T {
//   return { id, ...value };
// };
//
// function extractId<T extends WithId>(input: T): { id: number, value: Proto<T> } {
//   let { id, ...value } = input;
//   return { id, value };
// };
//
// type User = WithId & {
//   name: string;
//   email: string;
//   readonly created: Date;
// }
//
// type Ranking = {
//   readonly score: number;
//   readonly wins: number;
//   readonly losses: number;
//   readonly pointsWon: number;
//   readonly pointsLost: number;
// }
//
// type UserRanking = User & Ranking;
//
// type Items<T> = Status & {
//   items: T[];
// };
//
// type Status = {
//   pending: boolean;
//   error?: any;
// };
//
// const a: UserRanking = {
//   id: 3,
//   name: '',
//   email: '',
//   created: new Date(),
//   score: 2,
//   wins: 1,
//   losses: 1,
//   pointsWon: 1,
//   pointsLost: 1,
// };
//
// const b: Proto<UserRanking> = {
//   name: '',
//   email: '',
//   created: new Date(),
//   score: 2,
//   wins: 1,
//   losses: 1,
//   pointsWon: 1,
//   pointsLost: 1,
// };
//
// const c: Proto<Ranking> = {
//   score: 2,
//   wins: 1,
//   losses: 1,
//   pointsWon: 1,
//   pointsLost: 1,
// };
//
// const d: User = {
//   id: 1,
//   name: '',
//   email: '',
//   created: new Date(),
// };
//
// const e: { id: number, user: Proto<User> } = (() => {
//   let { id, ...user } = d;
//   return { id, user };
// })();

type User = {
  readonly id: number;
  readonly name: string;
  readonly email: string;
  readonly score: number;
  readonly wins: number;
  readonly losses: number;
  readonly pointsWon: number;
  readonly pointsLost: number;
  readonly created: Date;
}

type UserTuple = ObjV

// type NewUser = Partial<Pick<User, 'name' | 'email'>>;
type New<T, N extends keyof T> = Partial<Pick<T, N>>;

function a(a: number, b: string) {
}

type A = Parameters<User>;

const b: A = [1, ''];
