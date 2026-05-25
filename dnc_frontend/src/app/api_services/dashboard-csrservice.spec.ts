import { TestBed } from '@angular/core/testing';

import { DashboardCSRService } from './dashboard-csrservice';

describe('DashboardCSRService', () => {
  let service: DashboardCSRService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DashboardCSRService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
