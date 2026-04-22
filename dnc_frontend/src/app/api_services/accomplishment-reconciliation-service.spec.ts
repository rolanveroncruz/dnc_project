import { TestBed } from '@angular/core/testing';

import { AccomplishmentReconciliationService } from './accomplishment-reconciliation-service';

describe('AccomplishmentReconciliationService', () => {
  let service: AccomplishmentReconciliationService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(AccomplishmentReconciliationService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
