import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupHMOs } from './setup-hmos';

describe('SetupHMOs', () => {
  let component: SetupHMOs;
  let fixture: ComponentFixture<SetupHMOs>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupHMOs]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupHMOs);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
