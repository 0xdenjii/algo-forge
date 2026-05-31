/**
 * Title: Native Prototypal Creation via Object.create
 * Problem: Demonstrate JavaScript's built-in prototype pattern: derive objects from
 *   a base prototype, sharing methods while keeping own state.
 * Approach: Object.create(proto) + Object.assign for own fields; methods live on proto.
 * Complexity: O(1) creation; method lookup walks the prototype chain.
 */
interface Animal { name: string; sound: string; speak(): string }
const animalProto: Pick<Animal, "speak"> = {
  speak(this: Animal) { return `${this.name} says ${this.sound}`; },
};
function makeAnimal(name: string, sound: string): Animal {
  return Object.assign(Object.create(animalProto) as Animal, { name, sound });
}

const dog = makeAnimal("Rex", "woof");
const cat = makeAnimal("Tom", "meow");
console.log(dog.speak(), "|", cat.speak(), Object.getPrototypeOf(dog) === animalProto);
