import { TestBed } from '@angular/core/testing';

import { ClinicCapabilitiesService } from './clinic-capabilities-service';

describe('ClinicCapabilitiesService', () => {
  let service: ClinicCapabilitiesService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ClinicCapabilitiesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
