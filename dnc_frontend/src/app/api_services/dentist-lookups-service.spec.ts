import { TestBed } from '@angular/core/testing';

import { DentistLookupsService } from './dentist-lookups-service';

describe('DentistLookupsService', () => {
  let service: DentistLookupsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DentistLookupsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
