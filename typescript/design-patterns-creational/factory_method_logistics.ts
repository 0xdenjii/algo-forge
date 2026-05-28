/**
 * Title: Factory Method (Logistics)
 * Problem: A Logistics app plans deliveries; RoadLogistics uses Trucks, SeaLogistics
 *   uses Ships. Shared planning logic must not depend on the concrete transport.
 * Approach: abstract creator declares createTransport(); subclasses override it.
 * Complexity: O(1).
 */
interface Transport { deliver(cargo: string): string }
class Truck implements Transport { deliver(c: string) { return `Truck drives ${c} by road`; } }
class Ship implements Transport { deliver(c: string) { return `Ship carries ${c} by sea`; } }

abstract class Logistics {
  protected abstract createTransport(): Transport;
  planDelivery(cargo: string): string {
    return this.createTransport().deliver(cargo);
  }
}
class RoadLogistics extends Logistics { protected createTransport() { return new Truck(); } }
class SeaLogistics extends Logistics { protected createTransport() { return new Ship(); } }

for (const l of [new RoadLogistics(), new SeaLogistics()]) console.log(l.planDelivery("bikes"));
