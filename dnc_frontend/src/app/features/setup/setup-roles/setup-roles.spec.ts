import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupRoles } from './setup-roles';

describe('SetupRoles', () => {
  let component: SetupRoles;
  let fixture: ComponentFixture<SetupRoles>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupRoles]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupRoles);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
