import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddEditRoles } from './add-edit-roles';

describe('AddEditRoles', () => {
  let component: AddEditRoles;
  let fixture: ComponentFixture<AddEditRoles>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddEditRoles]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddEditRoles);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
