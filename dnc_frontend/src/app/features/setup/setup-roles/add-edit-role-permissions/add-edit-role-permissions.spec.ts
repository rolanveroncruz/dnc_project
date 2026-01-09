import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddEditRolePermissions } from './add-edit-role-permissions.dialog';

describe('AddEditRolePermissions', () => {
  let component: AddEditRolePermissions;
  let fixture: ComponentFixture<AddEditRolePermissions>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddEditRolePermissions]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddEditRolePermissions);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
