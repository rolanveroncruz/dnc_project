import { TestBed } from '@angular/core/testing';

import { WebsiteApplicationsService } from './website-applications-service';

describe('WebsiteApplicationsService', () => {
  let service: WebsiteApplicationsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(WebsiteApplicationsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
