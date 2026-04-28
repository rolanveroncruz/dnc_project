import { TestBed } from '@angular/core/testing';

import { UtilizationReportsService } from './utilization-reports-service';

describe('UtilizationReportsService', () => {
  let service: UtilizationReportsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(UtilizationReportsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
