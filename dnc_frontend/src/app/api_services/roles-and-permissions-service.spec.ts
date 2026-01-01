import { TestBed } from '@angular/core/testing';

import { RolesAndPermissionsService } from './roles-and-permissions-service';

describe('RolesAndPermissionsService', () => {
  let service: RolesAndPermissionsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(RolesAndPermissionsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
