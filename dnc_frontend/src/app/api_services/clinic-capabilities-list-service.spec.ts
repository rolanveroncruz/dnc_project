import { TestBed } from '@angular/core/testing';

import { ClinicCapabilitiesListService } from './clinic-capabilities-list-service';

describe('ClinicCapabilitiesListService', () => {
  let service: ClinicCapabilitiesListService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ClinicCapabilitiesListService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
